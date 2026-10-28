#[derive(Debug, Clone)]
pub enum BrowserEvent {
    Navigate(String),
    GoBack,
    GoForward,
    Reload,
    NewTab(String),
    CloseTab(usize),
    SwitchTab(usize),
    ToggleShield(Option<bool>),
    ToggleBookmark,
    ClearHistory,
    DeleteBookmark(i64),
    PageLoaded(String),
    TitleChanged(String),
}
