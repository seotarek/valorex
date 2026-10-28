use adblock::request::Request;
use adblock::Engine;
use log::{info, warn};

/// Valorex Shield Engine - Core ad and tracker blocker built on the official Brave Adblock syntax.
pub struct ShieldEngine {
    engine: Engine,
    enabled: bool,
    blocked_count: u64,
}

impl Default for ShieldEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ShieldEngine {
    /// Creates a new ShieldEngine populated with standard built-in privacy protection rules.
    pub fn new() -> Self {
        let built_in_rules = [
            "||doubleclick.net^",
            "||google-analytics.com^",
            "||googlesyndication.com^",
            "||adservice.google.com^",
            "||facebook.com/tr/^$script",
            "||connect.facebook.net/*/fbevents.js",
            "||taboola.com^",
            "||outbrain.com^",
            "||scorecardresearch.com^",
            "||quantserve.com^",
            "||adnxs.com^",
            "||criteo.com^",
            "||adroll.com^",
            "||amazon-adsystem.com^",
            "||hotjar.com^",
            "||yandex.ru/metrika^",
            "||mc.yandex.ru^",
        ];

        let engine = Engine::new_with_list_text(built_in_rules.join("\n"));
        info!("Valorex Shield initialized with {} built-in privacy rules.", built_in_rules.len());

        Self {
            engine,
            enabled: true,
            blocked_count: 0,
        }
    }

    /// Appends custom filter rules (e.g. EasyList, EasyPrivacy lines)
    pub fn load_rules<I, S>(&mut self, rules: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let rules_text = rules
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        self.engine = Engine::new_with_list_text(rules_text);
    }

    /// Evaluates whether an outgoing web request should be blocked.
    pub fn should_block(&mut self, url: &str, source_url: &str, request_type: &str) -> bool {
        if !self.enabled {
            return false;
        }

        match Request::new(url, source_url, request_type, "GET") {
            Ok(request) => {
                let check_result = self.engine.check_network_request(&request);
                if check_result.should_block() {
                    self.blocked_count += 1;
                    warn!("[Shield Blocked] URL: {} (Type: {})", url, request_type);
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_blocked_count(&self) -> u64 {
        self.blocked_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_blocks_doubleclick_ads() {
        let mut shield = ShieldEngine::new();
        assert!(shield.is_enabled());

        let is_blocked = shield.should_block(
            "https://adservice.google.com/adsid/google/ui",
            "https://example.com",
            "script",
        );
        assert!(is_blocked, "Expected Google adservice to be blocked by Valorex Shield");
    }

    #[test]
    fn test_shield_blocks_facebook_tracking_pixel() {
        let mut shield = ShieldEngine::new();
        let is_blocked = shield.should_block(
            "https://connect.facebook.net/en_US/fbevents.js",
            "https://store.example.com",
            "script",
        );
        assert!(is_blocked, "Expected Facebook tracking pixel to be blocked");
    }

    #[test]
    fn test_shield_allows_benign_content() {
        let mut shield = ShieldEngine::new();
        let is_blocked = shield.should_block(
            "https://cdn.example.com/assets/logo.png",
            "https://example.com",
            "image",
        );
        assert!(!is_blocked, "Expected legitimate asset to be allowed");
    }

    #[test]
    fn test_shield_toggle_disabling() {
        let mut shield = ShieldEngine::new();
        shield.set_enabled(false);

        let is_blocked = shield.should_block(
            "https://doubleclick.net/tracker.js",
            "https://example.com",
            "script",
        );
        assert!(!is_blocked, "Expected ad to be allowed when Shield is toggled OFF");
    }
}
