use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub blocked_count: u64,
}

pub struct TabManager {
    tabs: Vec<Tab>,
    active_id: usize,
    next_id: usize,
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new("valorex://newtab")
    }
}

impl TabManager {
    pub fn new(initial_url: &str) -> Self {
        let initial_tab = Tab {
            id: 1,
            title: "New Tab".to_string(),
            url: initial_url.to_string(),
            can_go_back: false,
            can_go_forward: false,
            blocked_count: 0,
        };
        Self {
            tabs: vec![initial_tab],
            active_id: 1,
            next_id: 2,
        }
    }

    pub fn new_tab(&mut self, url: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let tab = Tab {
            id,
            title: "New Tab".to_string(),
            url: url.to_string(),
            can_go_back: false,
            can_go_forward: false,
            blocked_count: 0,
        };
        self.tabs.push(tab);
        self.active_id = id;
        id
    }

    pub fn close_tab(&mut self, id: usize) -> Option<usize> {
        if self.tabs.len() <= 1 {
            // Keep at least one tab open
            if let Some(tab) = self.tabs.first_mut() {
                tab.title = "New Tab".to_string();
                tab.url = "valorex://newtab".to_string();
                tab.blocked_count = 0;
            }
            return Some(self.active_id);
        }

        if let Some(pos) = self.tabs.iter().position(|t| t.id == id) {
            self.tabs.remove(pos);
            if self.active_id == id {
                let new_active_index = if pos >= self.tabs.len() {
                    self.tabs.len() - 1
                } else {
                    pos
                };
                self.active_id = self.tabs[new_active_index].id;
            }
            Some(self.active_id)
        } else {
            None
        }
    }

    pub fn switch_tab(&mut self, id: usize) -> bool {
        if self.tabs.iter().any(|t| t.id == id) {
            self.active_id = id;
            true
        } else {
            false
        }
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.iter().find(|t| t.id == self.active_id)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        let active_id = self.active_id;
        self.tabs.iter_mut().find(|t| t.id == active_id)
    }

    pub fn active_id(&self) -> usize {
        self.active_id
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn get_tab_json_list(&self) -> Vec<serde_json::Value> {
        self.tabs
            .iter()
            .map(|t| {
                serde_json::json!({
                    "id": t.id,
                    "title": t.title,
                    "url": t.url,
                    "active": t.id == self.active_id,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_lifecycle() {
        let mut manager = TabManager::new("https://duckduckgo.com");
        assert_eq!(manager.tabs().len(), 1);
        assert_eq!(manager.active_id(), 1);

        let t2 = manager.new_tab("https://rust-lang.org");
        assert_eq!(manager.tabs().len(), 2);
        assert_eq!(manager.active_id(), t2);

        manager.switch_tab(1);
        assert_eq!(manager.active_id(), 1);

        manager.close_tab(t2);
        assert_eq!(manager.tabs().len(), 1);
        assert_eq!(manager.active_id(), 1);

        // Closing last remaining tab resets it rather than destroying it
        manager.close_tab(1);
        assert_eq!(manager.tabs().len(), 1);
        assert_eq!(manager.tabs()[0].url, "valorex://newtab");
    }
}
