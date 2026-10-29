use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Arabic,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Arabic
    }
}

impl Language {
    pub fn from_code(code: &str) -> Self {
        let lower = code.trim().to_lowercase();
        if lower.starts_with("en") {
            Language::English
        } else {
            Language::Arabic
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::Arabic => "ar",
            Language::English => "en",
        }
    }

    pub fn is_rtl(&self) -> bool {
        matches!(self, Language::Arabic)
    }

    pub fn direction(&self) -> &'static str {
        if self.is_rtl() {
            "rtl"
        } else {
            "ltr"
        }
    }

    // --- Tab and Window UI Strings ---
    pub fn new_tab(&self) -> &'static str {
        match self {
            Language::Arabic => "علامة تبويب جديدة",
            Language::English => "New Tab",
        }
    }

    pub fn close_tab(&self) -> &'static str {
        match self {
            Language::Arabic => "إغلاق التبويب",
            Language::English => "Close Tab",
        }
    }

    pub fn close_other_tabs(&self) -> &'static str {
        match self {
            Language::Arabic => "إغلاق التبويبات الأخرى",
            Language::English => "Close Other Tabs",
        }
    }

    pub fn close_tabs_to_right(&self) -> &'static str {
        match self {
            Language::Arabic => "إغلاق التبويبات إلى اليسار",
            Language::English => "Close Tabs to the Right",
        }
    }

    pub fn duplicate_tab(&self) -> &'static str {
        match self {
            Language::Arabic => "تكرار التبويب",
            Language::English => "Duplicate Tab",
        }
    }

    pub fn pin_tab(&self) -> &'static str {
        match self {
            Language::Arabic => "تثبيت التبويب",
            Language::English => "Pin Tab",
        }
    }

    pub fn unpin_tab(&self) -> &'static str {
        match self {
            Language::Arabic => "إلغاء تثبيت التبويب",
            Language::English => "Unpin Tab",
        }
    }

    pub fn reload(&self) -> &'static str {
        match self {
            Language::Arabic => "إعادة تحميل",
            Language::English => "Reload",
        }
    }

    pub fn reopen_closed_tab(&self) -> &'static str {
        match self {
            Language::Arabic => "إعادة فتح التبويب المغلق",
            Language::English => "Reopen Closed Tab",
        }
    }

    // --- Window Controls ---
    pub fn toggle_window_controls(&self) -> &'static str {
        match self {
            Language::Arabic => "أزرار التحكم بالنافذة",
            Language::English => "Window Caption Controls",
        }
    }

    pub fn toggle_tab_close_btn(&self) -> &'static str {
        match self {
            Language::Arabic => "زر إغلاق التبويب",
            Language::English => "Tab Close (X) Button",
        }
    }

    pub fn toggle_system_titlebar(&self) -> &'static str {
        match self {
            Language::Arabic => "شريط عنوان النظام الأصلي",
            Language::English => "Use System Title Bar",
        }
    }

    // --- Menu & Navigation Strings ---
    pub fn history(&self) -> &'static str {
        match self {
            Language::Arabic => "سجل التصفح",
            Language::English => "Browsing History",
        }
    }

    pub fn bookmarks(&self) -> &'static str {
        match self {
            Language::Arabic => "الإشارات المرجعية والمفضلة",
            Language::English => "Bookmarks",
        }
    }

    pub fn downloads(&self) -> &'static str {
        match self {
            Language::Arabic => "التنزيلات",
            Language::English => "Downloads",
        }
    }

    pub fn settings(&self) -> &'static str {
        match self {
            Language::Arabic => "الإعدادات",
            Language::English => "Settings",
        }
    }

    pub fn google_login(&self) -> &'static str {
        match self {
            Language::Arabic => "تسجيل الدخول بحساب Google",
            Language::English => "Sign in to Google Account",
        }
    }

    pub fn gemini_ai(&self) -> &'static str {
        match self {
            Language::Arabic => "مساعد Gemini الذكي",
            Language::English => "Gemini AI Copilot",
        }
    }

    pub fn import_data(&self) -> &'static str {
        match self {
            Language::Arabic => "استيراد البيانات والمفضلة",
            Language::English => "Import Browser Data",
        }
    }

    pub fn switch_language(&self) -> &'static str {
        match self {
            Language::Arabic => "التبديل إلى الإنجليزية",
            Language::English => "Switch to Arabic",
        }
    }

    pub fn exit_browser(&self) -> &'static str {
        match self {
            Language::Arabic => "خروج من المتصفح",
            Language::English => "Exit Valorex",
        }
    }

    pub fn search_placeholder(&self) -> &'static str {
        match self {
            Language::Arabic => "ابحث باستخدام Google أو أدخل رابط موقع...",
            Language::English => "Search with Google or enter web address...",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_defaults_and_switching() {
        let lang_ar = Language::from_code("ar");
        assert_eq!(lang_ar, Language::Arabic);
        assert!(lang_ar.is_rtl());
        assert_eq!(lang_ar.direction(), "rtl");
        assert_eq!(lang_ar.new_tab(), "علامة تبويب جديدة");

        let lang_en = Language::from_code("en");
        assert_eq!(lang_en, Language::English);
        assert!(!lang_en.is_rtl());
        assert_eq!(lang_en.direction(), "ltr");
        assert_eq!(lang_en.new_tab(), "New Tab");
    }
}
