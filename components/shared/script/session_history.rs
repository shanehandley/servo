/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cmp::{Ord, PartialOrd};
use std::sync::atomic::{AtomicUsize, Ordering};

use malloc_size_of_derive::MallocSizeOf;
use serde::{Deserialize, Serialize};
use servo_url::ServoUrl;
use uuid::Uuid;

use crate::{ReferrerPolicy, StructuredSerializedData};

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, MallocSizeOf, PartialEq, Serialize)]
pub struct NestedHistoryId(usize);

impl NestedHistoryId {
    pub fn next() -> Self {
        static NEXT_NESTED_HISTORY_ID: AtomicUsize = AtomicUsize::new(0);
        Self(NEXT_NESTED_HISTORY_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for NestedHistoryId {
    fn default() -> Self {
        NestedHistoryId::next()
    }
}

/// <https://html.spec.whatwg.org/multipage/#nested-history>
#[derive(Clone, Debug, Default)]
pub struct NestedHistory {
    id: NestedHistoryId,
    entries: Vec<SessionHistoryEntry>
}

impl NestedHistory {
    pub fn id(&self) -> usize {
        self.id.0
    }

    pub fn entries(&self) -> Vec<SessionHistoryEntry> {
        self.entries.clone()
    }
}

/// <https://html.spec.whatwg.org/multipage/#document-state-2>
#[derive(Clone, Debug, Default)]
pub struct DocumentState {
    pub document_id: DocumentId,
    pub document_referrer_policy: ReferrerPolicy,
    pub reload_pending: bool,
    pub nested_histories: Vec<NestedHistory>,
}

/// <https://html.spec.whatwg.org/multipage/#she-step>
#[derive(Clone, Debug, Default)]
pub enum SessionHistoryEntryStep {
    #[default]
    /// The initial state
    Pending,
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
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SessionHistoryEntry {
    pub step: SessionHistoryEntryStep,
    url: ServoUrl,
    navigation_api_state: StructuredSerializedData,
    pub document_state: DocumentState,
    navigation_api_key: Uuid,
    scroll_restoration_mode: ScrollRestorationMode,
}

impl SessionHistoryEntry {
    pub fn new(
        navigation_api_state: StructuredSerializedData,
        url: Option<ServoUrl>,
    ) -> SessionHistoryEntry {
        SessionHistoryEntry {
            step: SessionHistoryEntryStep::default(),
            url: url.unwrap_or(ServoUrl::parse("about:blank").unwrap()),
            document_state: DocumentState::default(),
            navigation_api_state,
            navigation_api_key: Uuid::new_v4(),
            scroll_restoration_mode: ScrollRestorationMode::default(),
        }
    }

    pub fn navigation_api_key(&self) -> Uuid {
        self.navigation_api_key.clone()
    }

    pub fn navigation_api_state(&self) -> StructuredSerializedData {
        self.navigation_api_state.clone()
    }

    pub fn set_navigation_api_state(&mut self, state: StructuredSerializedData) {
        self.navigation_api_state = state;
    }
}

impl PartialEq for SessionHistoryEntry {
    fn eq(&self, other: &SessionHistoryEntry) -> bool {
        self.navigation_api_key == other.navigation_api_key
    }
}

impl Eq for SessionHistoryEntry {}

impl Ord for SessionHistoryEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.document_state.document_id.0
            .cmp(&other.document_state.document_id.0)
    }
}

impl PartialOrd for SessionHistoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
