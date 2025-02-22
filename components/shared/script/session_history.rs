/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::cell::RefCell;
use std::cmp::{Ord, PartialOrd};
use std::sync::atomic::{AtomicUsize, Ordering};

use malloc_size_of_derive::MallocSizeOf;
use serde::{Deserialize, Serialize};
use servo_url::{ImmutableOrigin, MutableOrigin, ServoUrl};
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
    entries: Vec<SessionHistoryEntry>,
}

impl NestedHistory {
    pub fn id(&self) -> usize {
        self.id.0
    }

    pub fn entries(&self) -> Vec<SessionHistoryEntry> {
        self.entries.clone()
    }
}

/// Holds state inside a session history entry regarding how to present and, if necessary, recreate,
/// a Document. 
///
/// <https://html.spec.whatwg.org/multipage/#document-state-2>
#[derive(Clone, Debug)]
pub struct DocumentState {
    pub document_id: DocumentId,
    pub document_referrer_policy: ReferrerPolicy,
    pub reload_pending: bool,
    /// A list of nested histories, initially an empty list.
    pub nested_histories: Vec<NestedHistory>,
    pub navigable_target_name: Option<String>,
    pub initiator_origin: Option<MutableOrigin>,
    pub origin: ImmutableOrigin,
    pub about_base_url: Option<ServoUrl>,
    pub request_referrer_policy: ReferrerPolicy
}


impl DocumentState {
    pub fn new(
        document_id: DocumentId,
        document_referrer_policy: ReferrerPolicy,
        navigable_target_name: Option<String>,
        initiator_origin: Option<MutableOrigin>,
        origin: ImmutableOrigin,
        about_base_url: Option<ServoUrl>,
    ) -> DocumentState {
        DocumentState {
            document_id,
            document_referrer_policy,
            reload_pending: false,
            nested_histories: vec![],
            navigable_target_name,
            initiator_origin,
            origin,
            about_base_url,
            request_referrer_policy: ReferrerPolicy::default(),
        }
    }
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
    pub step: RefCell<SessionHistoryEntryStep>,
    url: ServoUrl,
    navigation_api_state: Option<StructuredSerializedData>,
    pub document_state: DocumentState,
    navigation_api_key: Uuid,
    scroll_restoration_mode: ScrollRestorationMode,
}

impl SessionHistoryEntry {
    pub fn new(url: ServoUrl, document_state: DocumentState) -> SessionHistoryEntry {
        SessionHistoryEntry {
            step: RefCell::new(SessionHistoryEntryStep::default()),
            url,
            document_state,
            navigation_api_state: None,
            navigation_api_key: Uuid::new_v4(),
            scroll_restoration_mode: ScrollRestorationMode::default(),
        }
    }

    pub fn set_step(&self, step: usize) {
        *self.step.borrow_mut() = SessionHistoryEntryStep::Integer(step)
    }

    pub fn navigation_api_key(&self) -> Uuid {
        self.navigation_api_key.clone()
    }

    pub fn navigation_api_state(&self) -> Option<StructuredSerializedData> {
        self.navigation_api_state.clone()
    }

    pub fn set_navigation_api_state(&mut self, state: StructuredSerializedData) {
        self.navigation_api_state = Some(state);
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
        self.document_state
            .document_id
            .0
            .cmp(&other.document_state.document_id.0)
    }
}

impl PartialOrd for SessionHistoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
