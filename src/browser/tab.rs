use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub blocked_count: u64,
    pub is_pinned: bool,
}

pub struct TabManager {
    tabs: Vec<Tab>,
    active_id: usize,
    next_id: usize,
    closed_tabs: Vec<String>,
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
            is_pinned: false,
        };
        Self {
            tabs: vec![initial_tab],
            active_id: 1,
            next_id: 2,
            closed_tabs: Vec::new(),
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
            is_pinned: false,
        };
        self.tabs.push(tab);
        self.active_id = id;
        id
    }

    pub fn duplicate_tab(&mut self, id: usize) -> Option<usize> {
        let original = self.tabs.iter().find(|t| t.id == id)?.clone();
        let new_id = self.next_id;
        self.next_id += 1;

        let new_tab = Tab {
            id: new_id,
            title: original.title,
            url: original.url,
            can_go_back: false,
            can_go_forward: false,
            blocked_count: 0,
            is_pinned: false,
        };

        let insert_idx = self
            .tabs
            .iter()
            .position(|t| t.id == id)
            .map(|pos| pos + 1)
            .unwrap_or(self.tabs.len());

        self.tabs.insert(insert_idx, new_tab);
        self.active_id = new_id;
        Some(new_id)
    }

    pub fn toggle_pin_tab(&mut self, id: usize) -> bool {
        if let Some(pos) = self.tabs.iter().position(|t| t.id == id) {
            let was_pinned = self.tabs[pos].is_pinned;
            self.tabs[pos].is_pinned = !was_pinned;
            let now_pinned = self.tabs[pos].is_pinned;

            let tab = self.tabs.remove(pos);
            if now_pinned {
                let first_unpinned = self.tabs.iter().position(|t| !t.is_pinned).unwrap_or(self.tabs.len());
                self.tabs.insert(first_unpinned, tab);
            } else {
                let last_pinned = self.tabs.iter().rposition(|t| t.is_pinned).map(|p| p + 1).unwrap_or(0);
                self.tabs.insert(last_pinned, tab);
            }
            now_pinned
        } else {
            false
        }
    }

    pub fn close_other_tabs(&mut self, keep_id: usize) {
        if self.tabs.iter().any(|t| t.id == keep_id) {
            for tab in &self.tabs {
                if tab.id != keep_id && !tab.is_pinned && tab.url != "valorex://newtab" {
                    self.closed_tabs.push(tab.url.clone());
                }
            }
            self.tabs.retain(|t| t.id == keep_id || t.is_pinned);
            self.active_id = keep_id;
        }
    }

    pub fn close_tabs_to_right(&mut self, id: usize) {
        if let Some(pos) = self.tabs.iter().position(|t| t.id == id) {
            let mut i = self.tabs.len();
            while i > pos + 1 {
                i -= 1;
                if !self.tabs[i].is_pinned {
                    let removed = self.tabs.remove(i);
                    if removed.url != "valorex://newtab" {
                        self.closed_tabs.push(removed.url);
                    }
                }
            }
            if !self.tabs.iter().any(|t| t.id == self.active_id) {
                self.active_id = id;
            }
        }
    }

    pub fn reopen_closed_tab(&mut self) -> Option<usize> {
        let url = self.closed_tabs.pop()?;
        Some(self.new_tab(&url))
    }

    pub fn reorder_tabs(&mut self, from_id: usize, to_id: usize) {
        if from_id == to_id {
            return;
        }
        let from_pos = self.tabs.iter().position(|t| t.id == from_id);
        let to_pos = self.tabs.iter().position(|t| t.id == to_id);

        if let (Some(fp), Some(tp)) = (from_pos, to_pos) {
            if self.tabs[fp].is_pinned != self.tabs[tp].is_pinned {
                return;
            }
            let tab = self.tabs.remove(fp);
            self.tabs.insert(tp, tab);
        }
    }

    pub fn next_tab(&mut self) -> usize {
        if self.tabs.is_empty() {
            return self.active_id;
        }
        let current_pos = self.tabs.iter().position(|t| t.id == self.active_id).unwrap_or(0);
        let next_pos = (current_pos + 1) % self.tabs.len();
        self.active_id = self.tabs[next_pos].id;
        self.active_id
    }

    pub fn prev_tab(&mut self) -> usize {
        if self.tabs.is_empty() {
            return self.active_id;
        }
        let current_pos = self.tabs.iter().position(|t| t.id == self.active_id).unwrap_or(0);
        let prev_pos = if current_pos == 0 {
            self.tabs.len() - 1
        } else {
            current_pos - 1
        };
        self.active_id = self.tabs[prev_pos].id;
        self.active_id
    }

    pub fn switch_to_index(&mut self, index: usize) -> Option<usize> {
        if index < self.tabs.len() {
            self.active_id = self.tabs[index].id;
            Some(self.active_id)
        } else {
            None
        }
    }

    pub fn close_tab(&mut self, id: usize) -> Option<usize> {
        if self.tabs.len() <= 1 {
            // Keep at least one tab open
            if let Some(tab) = self.tabs.first_mut() {
                if tab.id != id {
                    return None;
                }
                if tab.url != "valorex://newtab" {
                    self.closed_tabs.push(tab.url.clone());
                    if self.closed_tabs.len() > 50 {
                        self.closed_tabs.remove(0);
                    }
                }
                tab.title = "New Tab".to_string();
                tab.url = "valorex://newtab".to_string();
                tab.blocked_count = 0;
                tab.is_pinned = false;
            }
            return Some(self.active_id);
        }

        if let Some(pos) = self.tabs.iter().position(|t| t.id == id) {
            let removed = self.tabs.remove(pos);
            if removed.url != "valorex://newtab" {
                self.closed_tabs.push(removed.url);
                if self.closed_tabs.len() > 50 {
                    self.closed_tabs.remove(0);
                }
            }
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
                    "pinned": t.is_pinned,
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

    #[test]
    fn test_duplicate_tab() {
        let mut manager = TabManager::new("https://google.com");
        let dup = manager.duplicate_tab(1).expect("Failed to duplicate tab");
        assert_eq!(manager.tabs().len(), 2);
        assert_eq!(manager.active_id(), dup);
        assert_eq!(manager.active_tab().unwrap().url, "https://google.com");
    }

    #[test]
    fn test_pin_tab() {
        let mut manager = TabManager::new("https://page1.com");
        let t2 = manager.new_tab("https://page2.com");
        assert!(!manager.tabs()[0].is_pinned);

        // Pin t2
        let pinned = manager.toggle_pin_tab(t2);
        assert!(pinned);
        // t2 should now be at the front
        assert_eq!(manager.tabs()[0].id, t2);
        assert!(manager.tabs()[0].is_pinned);

        // Unpin t2
        let unpinned = manager.toggle_pin_tab(t2);
        assert!(!unpinned);
    }

    #[test]
    fn test_close_other_tabs() {
        let mut manager = TabManager::new("https://page1.com");
        let t2 = manager.new_tab("https://page2.com");
        let _t3 = manager.new_tab("https://page3.com");
        assert_eq!(manager.tabs().len(), 3);

        manager.close_other_tabs(t2);
        assert_eq!(manager.tabs().len(), 1);
        assert_eq!(manager.active_id(), t2);
    }

    #[test]
    fn test_reorder_tabs() {
        let mut manager = TabManager::new("https://page1.com");
        let t2 = manager.new_tab("https://page2.com");
        let t3 = manager.new_tab("https://page3.com");

        assert_eq!(manager.tabs()[0].id, 1);
        assert_eq!(manager.tabs()[1].id, t2);
        assert_eq!(manager.tabs()[2].id, t3);

        // Move t3 to first position
        manager.reorder_tabs(t3, 1);
        assert_eq!(manager.tabs()[0].id, t3);
        assert_eq!(manager.tabs()[1].id, 1);
        assert_eq!(manager.tabs()[2].id, t2);
    }

    #[test]
    fn test_cycle_tabs() {
        let mut manager = TabManager::new("https://page1.com");
        let t2 = manager.new_tab("https://page2.com");
        let t3 = manager.new_tab("https://page3.com");

        manager.switch_tab(1);
        assert_eq!(manager.next_tab(), t2);
        assert_eq!(manager.next_tab(), t3);
        assert_eq!(manager.next_tab(), 1);

        assert_eq!(manager.prev_tab(), t3);
        assert_eq!(manager.prev_tab(), t2);

        manager.switch_to_index(0);
        assert_eq!(manager.active_id(), 1);
    }

    #[test]
    fn test_close_tabs_to_right_and_reopen() {
        let mut manager = TabManager::new("https://tab1.com");
        let t2 = manager.new_tab("https://tab2.com");
        let _t3 = manager.new_tab("https://tab3.com");
        assert_eq!(manager.tabs().len(), 3);

        manager.close_tabs_to_right(t2);
        assert_eq!(manager.tabs().len(), 2);

        let reopened = manager.reopen_closed_tab().expect("Expected reopened tab");
        assert_eq!(manager.tabs().len(), 3);
        assert_eq!(manager.tabs().last().unwrap().id, reopened);
        assert_eq!(manager.tabs().last().unwrap().url, "https://tab3.com");
    }
}
