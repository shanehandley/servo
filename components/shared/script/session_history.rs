use servo_url::ServoUrl;

/// <https://html.spec.whatwg.org/multipage/#document-state-2>
#[derive(Debug, Default)]
pub struct SessionHistoryEntryDocumentState {
    /// todo
    pub reload_pending: bool,
}

/// <https://html.spec.whatwg.org/multipage/browsing-the-web.html#she-step>
#[derive(Debug, Default)]
pub enum SessionHistoryEntryStep {
    #[default]
    /// The initial state
    Pending,
    /// todo
    Integer(usize),
}

/// <https://html.spec.whatwg.org/multipage/#scroll-restoration-mode>
#[derive(Debug, Default)]
pub enum ScrollRestorationMode {
    /// The user agent is responsible for restoring the scroll position upon navigation.
    #[default]
    Auto,
    /// The page is responsible for restoring the scroll position and the user agent does not
    /// attempt to do so automatically
    Manual,
}

/// <https://html.spec.whatwg.org/multipage/#session-history-entry>
#[derive(Debug)]
pub struct SessionHistoryEntry {
    step: SessionHistoryEntryStep,
    url: ServoUrl,
    document_state: SessionHistoryEntryDocumentState,
    scroll_restoration_mode: ScrollRestorationMode,
}

impl Default for SessionHistoryEntry {
    fn default() -> Self {
        Self {
            step: Default::default(),
            url: ServoUrl::parse("about:blank").unwrap(),
            document_state: Default::default(),
            scroll_restoration_mode: ScrollRestorationMode::Auto,
        }
    }
}
