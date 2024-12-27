/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;
use std::cmp::Eq;
use indexmap::IndexMap;
use servo_atoms::Atom;

use dom_struct::dom_struct;
use servo_url::ImmutableOrigin;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use crate::dom::bindings::codegen::Bindings::NavigationBinding::{
    NavigationUpdateCurrentEntryOptions, NavigationMethods, NavigationNavigateOptions,
    NavigationResult, NavigationOptions, NavigationReloadOptions
};
use crate::dom::bindings::codegen::Bindings::NavigationCurrentEntryChangeEventBinding::NavigationCurrentEntryChangeEventInit;
use crate::dom::bindings::codegen::Bindings::NavigationHistoryEntryBinding::
    NavigationHistoryEntry_Binding::NavigationHistoryEntryMethods;
use crate::dom::bindings::codegen::Bindings::WindowBinding::Window_Binding::WindowMethods;
use crate::dom::bindings::codegen::Bindings::EventBinding::EventInit;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::reflector::{DomObject, reflect_dom_object};
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::bindings::trace::RootedTraceableBox;
use crate::dom::eventtarget::EventTarget;
use crate::dom::globalscope::GlobalScope;
use crate::dom::navigateevent::NavigateEvent;
use crate::dom::navigationactivation::NavigationActivation;
use crate::dom::navigationcurrententrychangeevent::NavigationCurrentEntryChangeEvent;
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;
use crate::dom::navigationtransition::NavigationTransition;
use crate::dom::promise::Promise;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigation-api-method-tracker>
#[derive(Clone, MallocSizeOf)]
struct NavigationApiMethodTracker {
    key: Option<String>,
    // #[ignore_malloc_size_of = "jsvalues are hard"]
    // info: JSValue,
    state: Option<String>, // TODO
    committed_to_entry: Option<DomRoot<NavigationHistoryEntry>>,
    #[ignore_malloc_size_of = "promises are hard"]
    committed_promise: Rc<Promise>,
    #[ignore_malloc_size_of = "promises are hard"]
    finished_promise: Rc<Promise>,
}

impl Eq for NavigationApiMethodTracker {}
impl PartialEq for NavigationApiMethodTracker {
    fn eq(&self, other: &Self) -> bool {
        self.key.is_some() && other.key.is_some() && self.key == other.key
    }
}

impl NavigationApiMethodTracker {
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#maybe-set-the-upcoming-non-traverse-api-method-tracker>
    pub fn new(
        global: &GlobalScope,
        // info: JSValue,
        state: Option<String>,
        committed_promise: Option<Rc<Promise>>,
        finished_promise: Option<Rc<Promise>>,
        can_gc: CanGc,
    ) -> NavigationApiMethodTracker {
        NavigationApiMethodTracker {
            key: None,
            // info,
            state,
            committed_to_entry: None,
            committed_promise: committed_promise.unwrap_or(Promise::new(global, can_gc)),
            finished_promise: finished_promise.unwrap_or(Promise::new(global, can_gc)),
        }
    }
}

#[dom_struct]
pub struct Navigation {
    event_target: EventTarget,
    window: Dom<Window>,
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigation-entry-list>
    entry_list: Vec<DomRoot<NavigationHistoryEntry>>,
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigation-current-entry-index>
    current_entry_index: Option<usize>,
    /// https://html.spec.whatwg.org/multipage/nav-history-apis.html#ongoing-navigate-event
    ongoing_event: Option<NavigateEvent>,
    // transition: Option<NavigationTransition>,
    focus_changed: bool,
    suppress_scroll: bool,
    #[no_trace]
    ongoing_method_tracker: Option<NavigationApiMethodTracker>,
    #[no_trace]
    /// <https://html.spec.whatwg.org/multipage/#upcoming-non-traverse-api-method-tracker>
    upcoming_non_traverse_method_tracker: DomRefCell<Option<NavigationApiMethodTracker>>,
    #[no_trace]
    #[ignore_malloc_size_of = "sets are hard"]
    /// An ordered map from strings to navigation API method trackers, initially empty.
    ///
    /// <https://html.spec.whatwg.org/multipage/#upcoming-traverse-api-method-trackers>
    upcoming_traverse_method_tracker: DomRefCell<IndexMap<String, NavigationApiMethodTracker>>,
}

impl Navigation {
    pub fn new(window: &Window) -> DomRoot<Navigation> {
        reflect_dom_object(
            Box::new(Navigation::new_inherited(window)),
            window,
            CanGc::note(),
        )
    }

    fn new_inherited(window: &Window) -> Navigation {
        Navigation {
            event_target: EventTarget::new_inherited(),
            window: Dom::from_ref(window),
            entry_list: vec![],
            current_entry_index: None,
            ongoing_event: None,
            focus_changed: false,
            suppress_scroll: false,
            ongoing_method_tracker: None,
            upcoming_non_traverse_method_tracker: DomRefCell::new(None),
            upcoming_traverse_method_tracker: DomRefCell::new(IndexMap::new()),
        }
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#has-entries-and-events-disabled>
    fn has_entries_and_events_disabled(&self) -> bool {
        // Step 1: Let document be navigation's relevant global object's associated Document.
        let document = &self.window.Document();

        // Step 2: If document is not fully active, then return true.
        if !document.is_fully_active() {
            return true;
        }

        // Step 3: If document's is initial about:blank is true, then return true.
        if document.url().as_str() == "about:blank" {
            // TODO is_initial_about_blank
            return true;
        }

        match document.origin().immutable() {
            // Step 4: If document's origin is opaque, then return true.
            ImmutableOrigin::Opaque(_) => true,
            // Step 5: Return false.
            _ => false,
        }
    }

    /// An early error result for an exception e is a NavigationResult dictionary instance given by
    /// «[ "committed" → a promise rejected with e, "finished" → a promise rejected with e ]».
    ///
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigation-api-early-error-result>
    fn early_error_result(&self, error: Error) -> NavigationResult {
        let mut result = NavigationResult::empty();

        let promise = Promise::new(&self.global(), CanGc::note());

        promise.reject_error(error);

        result.committed = Some(promise.clone());
        result.finished = Some(promise);

        result
    }

    /// A navigation API method tracker-derived result for a navigation API method tracker is a
    /// NavigationResult dictionary instance given by «[ "committed" → apiMethodTracker's committed
    /// promise, "finished" → apiMethodTracker's finished promise ]».
    ///
    /// <https://html.spec.whatwg.org/multipage/#navigation-api-method-tracker-derived-result>
    fn method_tracker_derived_result(&self, entry: NavigationApiMethodTracker) -> NavigationResult {
        let mut result = NavigationResult::empty();

        let promise = Promise::new(&self.global(), CanGc::note());

        promise.resolve_native(&result);

        result.committed = Some(entry.committed_promise);
        result.finished = Some(entry.finished_promise);

        result
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#performing-a-navigation-api-traversal>
    #[allow(unsafe_code)]
    fn perform_a_navigation_api_traversal(
        &self,
        key: DOMString,
        options: Option<RootedTraceableBox<NavigationOptions>>,
    ) -> NavigationResult {
        // Step 1. Let document be navigation's relevant global object's associated Document.
        let document = &self.window.Document();

        // Step 2. If document is not fully active, then return an early error result for an
        // "InvalidStateError" DOMException.
        if !document.is_fully_active() {
            return self.early_error_result(Error::InvalidState);
        }

        // Step 3. If document's unload counter is greater than 0, then return an early error result
        // for an "InvalidStateError" DOMException.
        // TODO

        // Step 4. Let current be the current entry of navigation.
        let current_entry = self.GetCurrentEntry();

        if let Some(entry) = current_entry {
            // Step 5. If key equals current's session history entry's navigation API key, then
            // return «[ "committed" → a promise resolved with current, "finished" → a promise
            // resolved with current ]».
            if entry.Key() == key {
                let mut result = NavigationResult::empty();

                let promise = Promise::new(&self.global(), CanGc::note());

                promise.resolve_native(&entry);

                result.committed = Some(promise.clone());
                result.finished = Some(promise);

                return result;
            }
        }

        let stringified_key = String::from(key);

        // Step 6. If navigation's upcoming traverse API method trackers[key] exists, then
        // return a navigation API method tracker-derived result for navigation's upcoming
        // traverse API method trackers[key].
        if let Some(entry) = self
            .upcoming_traverse_method_tracker
            .borrow()
            .get(&stringified_key)
        {
            return self.method_tracker_derived_result(entry.clone());
        }

        // Step 7. Let info be options["info"], if it exists; otherwise, undefined
        // let info = options.map(|o| o.info.to_owned());

        // Step 8. Let apiMethodTracker be the result of adding an upcoming traverse API method
        // tracker for navigation given key and info.
        let api_method_tracker = self.add_an_upcoming_traverse_api_method_tracker(stringified_key);

        // Step 9. Let navigable be document's node navigable
        // Step 10. Let traversable be navigable's traversable navigable.
        let navigable = document;

        // Step 11. Let sourceSnapshotParams be the result of snapshotting source snapshot params
        // given document.
        // let _snapshot_params = navigable.snapshot();

        // Step 12. Append the following session history traversal steps to traversable:
        // Step 12.1. Let navigableSHEs be the result of getting session history entries given
        // navigable.
        // let navigable_shes = document.get_session_history_entries().to_owned();

        // Step 12. Let targetSHE be the session history entry in navigableSHEs whose navigation API
        // key is key. If no such entry exists, then:

        // Step 12.2.1. Queue a global task on the navigation and traversal task source given
        // navigation's relevant global object to reject the finished promise for apiMethodTracker
        // with an "InvalidStateError" DOMException.

        // Step 13. Return a navigation API method tracker-derived result for apiMethodTracker.
        self.method_tracker_derived_result(api_method_tracker)
    }

    // TODO: info argument
    /// <https://html.spec.whatwg.org/multipage/#add-an-upcoming-traverse-api-method-tracker>
    fn add_an_upcoming_traverse_api_method_tracker(
        &self,
        key: String,
    ) -> NavigationApiMethodTracker {
        // Step 1. Let committedPromise and finishedPromise be new promises created in navigation's
        // relevant realm.
        let committed_promise = Promise::new(&self.global(), CanGc::note());
        let finished_promise = Promise::new(&self.global(), CanGc::note());

        // Step 2. Mark as handled finishedPromise.
        finished_promise.resolve_native(&());

        // Step 3. Let apiMethodTracker be a new navigation API method tracker with:
        let tracker = NavigationApiMethodTracker::new(
            &self.global(),
            // JSValue::new(),
            None,
            Some(committed_promise),
            Some(finished_promise),
            CanGc::note(),
        );

        // Step 4. Set navigation's upcoming traverse API method trackers[key] to apiMethodTracker.
        let (_, api_method_tracker) = self
            .upcoming_traverse_method_tracker
            .borrow_mut()
            .insert_sorted(key, tracker);

        // Step 5. Return apiMethodTracker.
        // TODO maybe make this fallable?
        api_method_tracker.unwrap()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#maybe-set-the-upcoming-non-traverse-api-method-tracker>
    fn maybe_set_the_upcoming_non_traverse_api_method_tracker(&self) -> NavigationApiMethodTracker {
        // Step 1. Let committedPromise and finishedPromise be new promises created in navigation's
        // relevant realm.
        let committed_promise = Promise::new(&self.global(), CanGc::note());
        let finished_promise = Promise::new(&self.global(), CanGc::note());

        // Step 2. Mark as handled finishedPromise.
        finished_promise.resolve_native(&());

        // Step 3. Let apiMethodTracker be a new navigation API method tracker with:
        let api_method_tracker = NavigationApiMethodTracker::new(
            &self.global(),
            // JSValue::new(),
            None,
            Some(committed_promise),
            Some(finished_promise),
            CanGc::note(),
        );

        // Step 4. Assert: navigation's upcoming non-traverse API method tracker is null.
        // debug_assert!(self.upcoming_non_traverse_method_tracker.borrow().is_none());

        // Step 5. If navigation does not have entries and events disabled, then set navigation's
        // upcoming non-traverse API method tracker to apiMethodTracker.
        if !self.has_entries_and_events_disabled() {
            *self.upcoming_non_traverse_method_tracker.borrow_mut() =
                Some(api_method_tracker.clone());
        }

        // Step 6. Return apiMethodTracker.
        api_method_tracker
    }
}

#[allow(non_snake_case)]
impl NavigationMethods<crate::DomTypeHolder> for Navigation {
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigation-entries>
    fn Entries(&self) -> Vec<DomRoot<NavigationHistoryEntry>> {
        // Step 1: If this has entries and events disabled, then return the empty list.
        if self.has_entries_and_events_disabled() {
            return vec![];
        }

        // Step 2: Return this's entry list.
        self.entry_list.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#navigation-current-entry>
    fn GetCurrentEntry(&self) -> Option<DomRoot<NavigationHistoryEntry>> {
        // Step 1: If navigation has entries and events disabled, then return null.
        if self.has_entries_and_events_disabled() {
            return None;
        }

        // Step 2, 3
        if let Some(idx) = self.current_entry_index {
            // Step 3: Return navigation's entry list[navigation's current entry index].
            return self.entry_list.get(idx).clone().cloned();
        }

        None
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigation-updatecurrententry>
    fn UpdateCurrentEntry(
        &self,
        options: RootedTraceableBox<NavigationUpdateCurrentEntryOptions>,
    ) -> Fallible<()> {
        // Step 1. Let current be the current entry of this.
        let current = self.GetCurrentEntry();

        // Step 2. If current is null, then throw an "InvalidStateError" DOMException.
        if current.is_none() {
            return Err(Error::InvalidState);
        }

        // Step 3. Let serializedState be StructuredSerializeForStorage(options["state"]),
        // rethrowing any exceptions.
        // TODO

        // Step 4. Set current's session history entry's navigation API state to serializedState.
        // TODO

        // Step 5. Fire an event named currententrychange at this using
        // NavigationCurrentEntryChangeEvent, with its navigationType attribute initialized to null
        // and its from initialized to current.
        let event_init = NavigationCurrentEntryChangeEventInit {
            parent: EventInit::empty(),
            navigationType: None,
            from: current.unwrap(),
        };

        let event = NavigationCurrentEntryChangeEvent::new(
            &self.window,
            None,
            Atom::from("currententrychange"),
            &event_init,
            CanGc::note(),
        );

        Ok(())
    }

    fn GetTransition(&self) -> Option<DomRoot<NavigationTransition>> {
        None
    }

    fn GetActivation(&self) -> Option<DomRoot<NavigationActivation>> {
        None
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigation-cangoback>
    fn CanGoBack(&self) -> bool {
        // Step 1. If this has entries and events disabled, then return false.
        if self.has_entries_and_events_disabled() {
            return false;
        }

        // Step 2. Assert: this's current entry index is not −1.

        // Step 3. If this's current entry index is 0, then return false.
        // Step 4. Return true.
        match self.current_entry_index {
            Some(idx) => idx > 0,
            _ => false,
        }
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigation-cangoforward>
    fn CanGoForward(&self) -> bool {
        // Step 1. If this has entries and events disabled, then return false.
        if self.has_entries_and_events_disabled() {
            return false;
        }

        // Step 2. Assert: this's current entry index is not −1.

        // Step 3. If this's current entry index is equal to this's entry list's size − 1, then
        // return false.
        // Step 4. Return true.
        match self.current_entry_index {
            Some(idx) => idx < self.entry_list.len(),
            _ => false,
        }
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigation-navigate>
    fn Navigate(
        &self,
        url: USVString,
        options: RootedTraceableBox<NavigationNavigateOptions>,
    ) -> NavigationResult {
        // Step 1

        // Step 2
        self.early_error_result(Error::Syntax)
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigation-reload>
    fn Reload(
        &self,
        options: RootedTraceableBox<NavigationReloadOptions>,
    ) -> Fallible<NavigationResult> {
        // Step 1. Let document be this's relevant global object's associated Document.
        let document = &self.window.Document();

        // Step 2. Let serializedState be StructuredSerializeForStorage(undefined).
        // TODO

        // Step 3. If options["state"] exists, then set serializedState to
        // StructuredSerializeForStorage(options["state"]). If this throws an exception, then return
        // an early error result for that exception.

        // Step 5. If document is not fully active, then return an early error result for an
        // "InvalidStateError" DOMException.
        if !document.is_fully_active() {
            return Err(Error::InvalidState);
        }

        // Step 8. Let apiMethodTracker be the result of maybe setting the upcoming non-traverse API
        // method tracker for this given info and serializedState.
        let api_method_tracker = self.maybe_set_the_upcoming_non_traverse_api_method_tracker();

        // Step 9. Reload document's node navigable with navigationAPIState set to serializedState.

        // Step 10. Return a navigation API method tracker-derived result for apiMethodTracker.
        Ok(self.method_tracker_derived_result(api_method_tracker))
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigation-traverseto>
    fn TraverseTo(
        &self,
        key: DOMString,
        options: RootedTraceableBox<NavigationOptions>,
    ) -> NavigationResult {
        // Step 1. If this's current entry index is −1, then return an early error result for an
        // "InvalidStateError" DOMException.
        match self.current_entry_index {
            None => self.early_error_result(Error::InvalidState),
            Some(i) if i < 1 || i == self.entry_list.len() => {
                self.early_error_result(Error::InvalidState)
            },
            Some(i) => {
                // Step 2. If this's entry list does not contain a NavigationHistoryEntry whose
                // session history entry's navigation API key equals key, then return an early error
                // result for an "InvalidStateError" DOMException.
                match self.entry_list.get(i + 1) {
                    Some(entry) => {
                        // Step 3. Return the result of performing a navigation API traversal given
                        // this, key, and options.
                        self.perform_a_navigation_api_traversal(entry.Key(), Some(options))
                    },
                    None => self.early_error_result(Error::InvalidState),
                }
            },
        }
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigation-back>
    fn Back(&self, options: RootedTraceableBox<NavigationOptions>) -> NavigationResult {
        // Step 1
        match self.current_entry_index {
            None => self.early_error_result(Error::InvalidState),
            Some(i) if i < 1 => self.early_error_result(Error::InvalidState),
            Some(i) => {
                // Step 2
                match self.entry_list.get(i - 1) {
                    Some(entry) => {
                        // Step 3
                        self.perform_a_navigation_api_traversal(entry.Key(), Some(options))
                    },
                    None => self.early_error_result(Error::InvalidState),
                }
            },
        }
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-navigation-forward>
    fn Forward(&self, options: RootedTraceableBox<NavigationOptions>) -> NavigationResult {
        // Step 1
        match self.current_entry_index {
            None => self.early_error_result(Error::InvalidState),
            Some(i) if i < 1 || i == self.entry_list.len() => {
                self.early_error_result(Error::InvalidState)
            },
            Some(i) => {
                // Step 2
                match self.entry_list.get(i + 1) {
                    Some(entry) => {
                        // Step 3
                        self.perform_a_navigation_api_traversal(entry.Key(), Some(options))
                    },
                    None => self.early_error_result(Error::InvalidState),
                }
            },
        }
    }

    // <https://html.spec.whatwg.org/multipage/nav-history-apis.html#handler-navigation-onnavigate>
    event_handler!(navigate, GetOnnavigate, SetOnnavigate);

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#handler-navigation-onnavigatesuccess>
    fn GetOnnavigatesuccess(&self) -> Option<Rc<EventHandlerNonNull>> {
        None
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#handler-navigation-onnavigatesuccess>
    fn SetOnnavigatesuccess(&self, value: Option<Rc<EventHandlerNonNull>>) {}

    // error_event_handler!(onnavigateerror, GetOnnavigateerror, SetOnnavigateerror)

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#handler-navigation-onnavigateerror>
    fn GetOnnavigateerror(&self) -> Option<Rc<EventHandlerNonNull>> {
        None
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#handler-navigation-onnavigateerror>
    fn SetOnnavigateerror(&self, value: Option<Rc<EventHandlerNonNull>>) {}

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#handler-navigation-oncurrententrychange>
    fn GetOncurrententrychange(&self) -> Option<Rc<EventHandlerNonNull>> {
        None
    }

    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#handler-navigation-oncurrententrychange>
    fn SetOncurrententrychange(&self, value: Option<Rc<EventHandlerNonNull>>) {}
}
