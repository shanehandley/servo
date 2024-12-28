/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::sync::atomic::{AtomicUsize, Ordering};

use malloc_size_of_derive::MallocSizeOf;
use serde::{Deserialize, Serialize};
use servo_url::ServoUrl;
use uuid::Uuid;

use crate::ReferrerPolicy;

/// Rather than actually copy a document into DocumentState, store a unique identifier to match it
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, MallocSizeOf, PartialEq, Serialize)]
pub struct DocumentId(usize);

impl DocumentId {
    pub fn next() -> Self {
        static NEXT_REQUEST_ID: AtomicUsize = AtomicUsize::new(0);
        Self(NEXT_REQUEST_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for DocumentId {
    fn default() -> Self {
        DocumentId::next()
    }
}

/// <https://html.spec.whatwg.org/multipage/#document-state-2>
#[derive(Clone, Debug, Default)]
pub struct DocumentState {
    pub document_id: DocumentId,
    pub document_referrer_policy: ReferrerPolicy,
    pub reload_pending: bool,
}

/// <https://html.spec.whatwg.org/multipage/browsing-the-web.html#she-step>
#[derive(Clone, Debug, Default)]
pub enum SessionHistoryEntryStep {
    #[default]
    /// The initial state
    Pending,
    /// todo
    Integer(usize),
}

/// <https://html.spec.whatwg.org/multipage/#scroll-restoration-mode>
#[derive(Clone, Debug, Default)]
pub enum ScrollRestorationMode {
    /// The user agent is responsible for restoring the scroll position upon navigation.
    #[default]
    Auto,
    /// The page is responsible for restoring the scroll position and the user agent does not
    /// attempt to do so automatically
    Manual,
}

/// <https://html.spec.whatwg.org/multipage/#session-history-entry>
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct SessionHistoryEntry {
    pub step: SessionHistoryEntryStep,
    url: ServoUrl,
    pub document_state: DocumentState,
    navigation_api_key: Uuid,
    scroll_restoration_mode: ScrollRestorationMode,
}

impl SessionHistoryEntry {
    pub fn navigation_api_key(&self) -> Uuid {
        self.navigation_api_key.clone()
    }
}

impl Default for SessionHistoryEntry {
    fn default() -> Self {
        Self {
            step: Default::default(),
            url: ServoUrl::parse("about:blank").unwrap(),
            document_state: Default::default(),
            navigation_api_key: Uuid::new_v4(),
            scroll_restoration_mode: ScrollRestorationMode::Auto,
        }
    }
}

impl PartialEq for SessionHistoryEntry {
    fn eq(&self, other: &SessionHistoryEntry) -> bool {
        self.navigation_api_key == other.navigation_api_key
    }
}
