pub mod updater;

use adblock::request::Request;
use adblock::Engine;
use log::{info, warn};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Valorex Shield Engine - Core ad, tracker, and cosmetic filter engine.
pub struct ShieldEngine {
    engine: Engine,
    enabled: bool,
    blocked_count: Arc<AtomicU64>,
    domain_block_counts: HashMap<String, u64>,
}

impl Default for ShieldEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ShieldEngine {
    pub const BUILT_IN_RULES: &'static [&'static str] = &[
        // Network blocking rules
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
        // Domain-specific cosmetic element hiding rules
        "example.com##.ad-container",
        "example.com##.sponsor-post",
        "twitter.com##[data-testid='placementTracking']",
        "youtube.com##.ytd-action-companion-ad-renderer",
    ];

    /// Creates a new ShieldEngine populated with standard built-in privacy protection and cosmetic rules.
    pub fn new() -> Self {
        let engine = Engine::new_with_list_text(Self::BUILT_IN_RULES.join("\n"));
        info!("Valorex Shield initialized with {} built-in privacy & cosmetic rules.", Self::BUILT_IN_RULES.len());

        Self {
            engine,
            enabled: true,
            blocked_count: Arc::new(AtomicU64::new(0)),
            domain_block_counts: HashMap::new(),
        }
    }

    /// Updates filter rules (e.g. from EasyList / EasyPrivacy) while preserving built-in rules
    pub fn load_rules<I, S>(&mut self, rules: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut combined = Self::BUILT_IN_RULES.join("\n");
        let external = rules
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        if !external.is_empty() {
            combined.push('\n');
            combined.push_str(&external);
        }
        self.engine = Engine::new_with_list_text(combined);
        info!("Valorex Shield updated with new filter rules (built-in rules preserved).");
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
                    self.blocked_count.fetch_add(1, Ordering::Relaxed);
                    if let Ok(parsed) = url::Url::parse(source_url) {
                        if let Some(host) = parsed.host_str() {
                            *self.domain_block_counts.entry(host.to_string()).or_insert(0) += 1;
                        }
                    }
                    warn!("[Shield Blocked] URL: {} (Type: {})", url, request_type);
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    /// Generates cosmetic CSS and JavaScript injection to hide ad placeholders on the current page.
    pub fn generate_cosmetic_script(&self, url: &str) -> Option<String> {
        if !self.enabled {
            return None;
        }

        let resources = self.engine.url_cosmetic_resources(url);
        let mut script_parts = Vec::new();

        if !resources.hide_selectors.is_empty() {
            let selectors = resources.hide_selectors.into_iter().collect::<Vec<_>>().join(", ");
            let css_snippet = format!("{} {{ display: none !important; }}", selectors);
            let css_json = serde_json::to_string(&css_snippet).unwrap_or_else(|_| "\"\"".to_string());
            let js_inject = format!(
                r#"
                (function() {{
                    const css = {};
                    let style = document.getElementById('valorex-shield-cosmetics');
                    if (!style) {{
                        style = document.createElement('style');
                        style.id = 'valorex-shield-cosmetics';
                        (document.head || document.documentElement).appendChild(style);
                    }}
                    style.textContent = css;
                }})();
                "#,
                css_json
            );
            script_parts.push(js_inject);
        }

        if !resources.injected_script.is_empty() {
            script_parts.push(resources.injected_script);
        }

        if script_parts.is_empty() {
            None
        } else {
            Some(script_parts.join("\n"))
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_blocked_count(&self) -> u64 {
        self.blocked_count.load(Ordering::Relaxed)
    }

    pub fn blocked_counter(&self) -> Arc<AtomicU64> {
        self.blocked_count.clone()
    }

    pub fn get_domain_blocked_count(&self, domain: &str) -> u64 {
        self.domain_block_counts.get(domain).copied().unwrap_or(0)
    }

    pub fn reset_domain_count(&mut self, domain: &str) {
        self.domain_block_counts.remove(domain);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_blocks_doubleclick_ads() {
        let mut shield = ShieldEngine::new();
        assert!(shield.should_block(
            "https://ad.doubleclick.net/pixel?id=123",
            "https://news.ycombinator.com",
            "script"
        ));
    }

    #[test]
    fn test_shield_blocks_facebook_tracking_pixel() {
        let mut shield = ShieldEngine::new();
        assert!(shield.should_block(
            "https://connect.facebook.net/en_US/fbevents.js",
            "https://shopping.example.com",
            "script"
        ));
    }

    #[test]
    fn test_shield_allows_benign_content() {
        let mut shield = ShieldEngine::new();
        assert!(!shield.should_block(
            "https://cdn.example.com/assets/logo.png",
            "https://example.com",
            "image"
        ));
    }

    #[test]
    fn test_shield_toggle_disabling() {
        let mut shield = ShieldEngine::new();
        shield.set_enabled(false);
        assert!(!shield.should_block(
            "https://ad.doubleclick.net/pixel",
            "https://example.com",
            "script"
        ));
    }

    #[test]
    fn test_cosmetic_script_generation() {
        let shield = ShieldEngine::new();
        let script = shield.generate_cosmetic_script("https://example.com");
        assert!(script.is_some());
        let code = script.unwrap();
        assert!(code.contains("display: none !important"));
        assert!(code.contains("valorex-shield-cosmetics"));
    }

    #[test]
    fn test_built_in_rules_preserved_on_load() {
        let mut shield = ShieldEngine::new();
        shield.load_rules(["||custom-tracker.org^"]);
        assert!(shield.should_block("https://custom-tracker.org/ping", "https://site.com", "script"));
        // Built-in rule must still block
        assert!(shield.should_block("https://ad.doubleclick.net/pixel", "https://site.com", "script"));
    }
}

