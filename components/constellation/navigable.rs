/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::todo;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::Weak;

use base::id::BrowsingContextId;
use script_traits::session_history::{DocumentId, DocumentState, SessionHistoryEntry};
use serde::{Deserialize, Serialize};
use servo_url::ServoUrl;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// An id to differeniate navigables.
pub struct NavigableId(usize);

impl Default for NavigableId {
    fn default() -> Self {
        Self(0)
    }
}

/// <https://html.spec.whatwg.org/multipage/#navigable>
pub struct Navigable {
    id: NavigableId,
    parent: Option<Weak<Navigable>>,
    is_closing: bool,
    active_session_history_entry: RefCell<Option<SessionHistoryEntry>>,
    current_session_history_entry: Option<SessionHistoryEntry>,
    /// A list of session history entries, initially a new list.
    session_history_entries: RefCell<Vec<SessionHistoryEntry>>,
    name: String,
}

// Perhaps Navigable is a trait, and `Traversable` implements it? Not sure when a non-traversable
// navigable comes into play
impl Navigable {
    /// Dependencies:
    /// 
    ///  - <https://html.spec.whatwg.org/multipage/document-sequences.html#browsing-context-group>
    ///  - storage shed: <https://storage.spec.whatwg.org/#legacy-clone-a-traversable-storage-shed>
    ///  - "A user agent holds a top-level traversable set (a set of top-level traversables).
    ///    These are typically presented to the user in the form of browser windows or browser tabs."
    ///    <https://html.spec.whatwg.org/multipage/document-sequences.html#top-level-traversable-set>
    /// 
    /// <https://html.spec.whatwg.org/multipage/#creating-a-new-top-level-traversable>
    pub fn new(
        opener: Option<BrowsingContextId>,
        target_name: Option<String>,
        opener_navigable: Option<Navigable>
    ) -> Navigable {
        todo!()

        // Step 1. Let document be null.

        // Step 2. If opener is null, then set document to the second return value of creating a
        // new top-level browsing context and document.

            // This process involves:

            // 1. Let group and document be the result of creating a new browsing context group and
            // document.

                // <https://html.spec.whatwg.org/multipage/document-sequences.html#browsing-context-group>

            // 2. Return group's browsing context set[0] and document.

        // Step 4. Let documentState be a new document state, with:
        // document: document
        // initiator origin:  null if opener is null; otherwise, document's origin
        // origin: document's origin
        // navigable target name: targetName
        // about base URL: document's about base URL

        // let document_state = DocumentState::new(
        //  document.id(),
        //  document.referrer_policy(),
        //  navigable_target_name: target_name,
        //  initiator_origin: None, //     null if opener is null; otherwise, document's origin
        //  about_base_url: document.about_base_url() // TODO
        //)

        // Step 5. Let traversable be a new traversable navigable.
        // let traversable = ...

        // Step 6. Initialize the navigable traversable given documentState.
        // self.initialize(document_state, None);

        // Step 7. Let initialHistoryEntry be traversable's active session history entry.
        // let initial_history_entry = traversable.active_session_history_entry()

        // Step 8. Set initialHistoryEntry's step to 0.
        // initial_history_entry.set_step(0)

        // Step 9. Append initialHistoryEntry to traversable's session history entries.
        // traversable.add_session_history_entry(initial_history_entry);

        // Step 10. If opener is non-null, then legacy-clone a traversable storage shed given
        // opener's top-level traversable and traversable.

            // <https://storage.spec.whatwg.org/#legacy-clone-a-traversable-storage-shed>

            // A traversable navigable holds a storage shed, which is a storage shed. A traversable
            // navigable’s storage shed holds all session storage data. 

            // Lot's to do there...

        // Step 11. Append traversable to the user agent's top-level traversable set.

            // ...

        // Step 12. Invoke WebDriver BiDi navigable created with traversable and
        // openerNavigableForWebDriver.

        // Step 13. Return traversable.
        // traversable
    }

    /// <https://html.spec.whatwg.org/multipage/#initialize-the-navigable>
    pub fn initialize(&self, document_state: DocumentState, parent: Option<Navigable>) {
        // Step 1. Assert: documentState's document is non-null.

        // Step 2. Let entry be a new session history entry, with
        // URL: documentState's document's URL
        // document state: documentState

        // let entry = SessionHistoryEntry::new(Some(document.url()), document_state);

        // Step 3. Set navigable's current session history entry to entry.
        // self.set_current_session_history_entry(entry);

        // Step 4. Set navigable's active session history entry to entry.
        // self.set_active_session_history_entry(entry);

        // Step 5. Set navigable's parent to parent.
        // self.set_parent(parent);
    }

    pub fn active_session_history_entry(&self) -> Option<SessionHistoryEntry> {
        self.active_session_history_entry.borrow().clone()
    }

    /// A navigable's active document is its active session history entry's document.
    ///
    /// <https://html.spec.whatwg.org/multipage/#nav-document>
    fn active_document(&self) -> Option<DocumentId> {
        let active_document_id = self.active_session_history_entry.borrow();

        None
    }

    /// A navigable's active browsing context is its active document's browsing context. If this
    /// navigable is a traversable navigable, then its active browsing context will be a top-level
    /// browsing context.
    ///
    /// <https://html.spec.whatwg.org/multipage/#nav-bc>
    pub fn active_browsing_context(&self) {
        // self.active_document().map(|document| doc.browsing_context())
    }

    /// A navigable's active WindowProxy is its active browsing context's associated WindowProxy.
    ///
    /// <https://html.spec.whatwg.org/multipage/#nav-wp>
    pub fn active_window_proxy(&self) {}

    /// A navigable's active window is its active WindowProxy's Window.
    ///
    /// <https://html.spec.whatwg.org/multipage/#nav-window>
    pub fn active_window(&self) {}

    /// A navigable's target name is its active session history entry's document state's navigable
    /// target name.
    ///
    /// <https://html.spec.whatwg.org/multipage/#nav-target>
    pub fn target_name(&self) -> Option<String> {
        None
    }

    /// <https://html.spec.whatwg.org/multipage/#getting-session-history-entries>
    // pub fn get_session_history_entries(&self) -> BTreeSet<SessionHistoryEntry> {
    //     // Step 1. Let traversable be navigable's traversable navigable.

    //     // Step 2. Assert: this is running within traversable's session history traversal queue.
    //     // TODO :o https://html.spec.whatwg.org/multipage/#tn-session-history-traversal-queue

    //     // Step 3. If navigable is traversable, return traversable's session history entries.
    //     self.session_history_entries.clone()

    //     // Step 4. Let docStates be an empty ordered set of document states.

    //     // Step 5. For each entry of traversable's session history entries, append entry's document
    //     // state to docStates.

    //     // Step 6. For each docState of docStates:
    //     // Step 6.1. For each nestedHistory of docState's nested histories:
    //     // Step 6.1.1. If nestedHistory's id equals navigable's id, return nestedHistory's entries.
    //     // Step 6.1.2. For each entry of nestedHistory's entries, append entry's document state to
    //     // docStates.

    //     // Step 7. Assert: this step is not reached.
    // }

    /// NavigationApi
    // TODO(NavigationAPI)
    /// <https://html.spec.whatwg.org/multipage/#apply-the-history-step>
    // pub(crate) fn apply_history_step(
    //     &self,
    //     step: usize,
    //     // check_for_cancellation: bool // TODO
    //     source_snapshot_params: Option<SourceSnapshotParams>,
    //     navigationType: Option<NavigationType>,
    // ) -> HistoryApplicationResult {
    //     // Step 1. Assert: This is running within traversable's session history traversal queue.
    //     // TODO

    //     // Step 2. Let targetStep be the result of getting the used step given traversable and step.
    //     let target_step = self.get_the_used_step(step);

    //     // Step 3. If initiatorToCheck is not null, then:
    //     // TODO

    //     // Step 4. Let navigablesCrossingDocuments be the result of getting all navigables that
    //     // might experience a cross-document traversal given traversable and targetStep.
    //     // https://html.spec.whatwg.org/multipage/#getting-all-navigables-that-might-experience-a-cross-document-traversal

    //     // Step 5. If checkForCancelation is true, and the result of checking if unloading is
    //     // canceled given navigablesCrossingDocuments, traversable, targetStep, and userInvolvement
    //     // is not "continue", then return that result.
    //     // https://html.spec.whatwg.org/multipage/#checking-if-unloading-is-canceled

    //     // Step 6. Let changingNavigables be the result of get all navigables whose current session
    //     // history entry will change or reload given traversable and targetStep.
    //     // https://html.spec.whatwg.org/multipage/#get-all-navigables-whose-current-session-history-entry-will-change-or-reload

    //     // Step 7. Let nonchangingNavigablesThatStillNeedUpdates be the result of getting all
    //     // navigables that only need history object length/index update given traversable and
    //     // targetStep.
    //     // https://html.spec.whatwg.org/multipage/#getting-all-navigables-that-only-need-history-object-length/index-update

    //     // Step 8. For each navigable of changingNavigables:
    //     // Step 8.1. Let targetEntry be the result of getting the target history entry given
    //     // navigable and targetStep.
    //     let target_entry = self.get_the_target_history_entry(step);

    //     // Step 8.2. Set navigable's current session history entry to targetEntry.
    //     if let Some(proxy) = self.browsing_context() {
    //         // proxy.set_current_history_entry(target_entry);
    //     }

    //     // Step 8.3. Set the ongoing navigation for navigable to "traversal".

    //     // Step 9. Let totalChangeJobs be the size of changingNavigables.
    //     let total_changed_jobs = 1;

    //     // TODO

    //     HistoryApplicationResult::Applied
    // }

    /// <https://html.spec.whatwg.org/multipage/#getting-the-used-step>
    // fn get_the_used_step(&self, step: usize) -> usize {
    //     // Step 1. Let steps be the result of getting all used history steps within traversable.
    //     let steps = self.get_all_used_history_steps();

    //     // Step 2. Return the greatest item in steps that is less than or equal to step.
    //     steps.range(..=step).next_back().cloned().unwrap_or(0)
    // }

    // TODO(NavigationAPI)
    /// <https://html.spec.whatwg.org/multipage/#getting-the-target-history-entry>
    // fn get_the_target_history_entry(&self, step: usize) -> SessionHistoryEntry {
    //     // Step 1. Let entries be the result of getting session history entries for navigable.
    //     let entries = self.get_session_history_entries();

    //     // Step 2. Return the item in entries that has the greatest step less than or equal to step.
    //     entries
    //         .iter()
    //         .filter(|entry| match entry.step {
    //             SessionHistoryEntryStep::Integer(i) => i <= step,
    //             _ => false,
    //         })
    //         .last()
    //         .expect("Document has no session history entries")
    //         .clone()
    // }

    // TODO(NavigationApi)
    /// <https://html.spec.whatwg.org/multipage/#getting-all-used-history-steps>
    fn get_all_used_history_steps(&self) -> Option<BTreeSet<usize>> {
        // // Step 2.1.1. Assert: this is running within traversable's session history traversal queue.
        // // TODO

        // // Step 2. Let steps be an empty ordered set of non-negative integers.
        // let mut steps: BTreeSet<usize> = BTreeSet::new();

        // // Step 3. Let entryLists be the ordered set « traversable's session history entries ».
        // let entry_list: BTreeSet<SessionHistoryEntry> = self.get_session_history_entries();

        // // It's not clear whether the entry_list should grow during iteration with values from
        // // entry.nested_histories? That would require two separate operations

        // for entry in entry_list.iter() {
        //     // Step 4.1.1. Append entry's step to steps.
        //     match entry.step {
        //         SessionHistoryEntryStep::Integer(value) => {
        //             steps.insert(value);
        //         },
        //         _ => {},
        //     }

        //     // For each nestedHistory of entry's document state's nested histories, append
        //     // nestedHistory's entries list to entryLists.
        //     for nested_history in entry.document_state.nested_histories.iter() {
        //         for entry in nested_history.entries().iter() {
        //             self.append_session_history_entry(entry.clone());
        //         }
        //     }
        // }

        // // Step 5. Return steps, sorted.
        // steps

        None
    }

    /// A top-level traversable is a traversable navigable with a null parent.
    ///
    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#top-level-traversable>
    pub fn is_top_level(&self) -> bool {
        self.parent.is_none()
    }
}
