/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;
// use js::jsapi::Heap;
// use js::jsval::JSVal;
use js::gc::{HandleObject, MutableHandleValue};
use servo_atoms::Atom;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::EventBinding::Event_Binding::EventMethods;
use crate::dom::bindings::codegen::Bindings::NavigateEventBinding::{
    NavigateEventInit, NavigateEventMethods, NavigationFocusReset, NavigationInterceptHandler,
    NavigationInterceptOptions, NavigationScrollBehavior,
};
use crate::dom::bindings::codegen::Bindings::NavigationBinding::NavigationType;
use crate::dom::bindings::codegen::Bindings::WindowBinding::Window_Binding::WindowMethods;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, DomGlobal};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::bindings::trace::RootedTraceableBox;
use crate::dom::event::{Event, EventStatus};
use crate::dom::formdata::FormData;
use crate::dom::navigationdestination::NavigationDestination;
use crate::dom::window::Window;
use crate::script_runtime::{CanGc, JSContext};

/// <https://html.spec.whatwg.org/multipage/#concept-navigateevent-interception-state>
#[derive(Clone, JSTraceable, MallocSizeOf, PartialEq)]
pub enum InterceptionState {
    None,
    Intercepted,
    Committed,
    Scrolled,
    Finished,
}

/// <https://html.spec.whatwg.org/multipage/#navigateevent>
#[dom_struct]
pub struct NavigateEvent {
    event: Event,
    // TODO
    // #[ignore_malloc_size_of = "mozjs"]
    // info: RootedTraceableBox<Heap<JSVal>>,
    navigation_type: NavigationType,
    destination: DomRoot<NavigationDestination>,
    interception_state: DomRefCell<InterceptionState>,
    #[ignore_malloc_size_of = "mozjs"]
    navigation_handler_list: DomRefCell<Vec<Rc<NavigationInterceptHandler>>>,
    focus_reset: DomRefCell<Option<NavigationFocusReset>>,
    scroll_behavior: DomRefCell<Option<NavigationScrollBehavior>>,
    download_request: DomRefCell<Option<DOMString>>,
    can_intercept: DomRefCell<bool>,
    user_initiated: DomRefCell<bool>,
    has_ua_visible_transitions: DomRefCell<bool>,
    hash_change: DomRefCell<bool>,
    form_data: Option<DomRoot<FormData>>,
}

impl NavigateEvent {
    fn new_inherited(init: &RootedTraceableBox<NavigateEventInit>) -> NavigateEvent {
        NavigateEvent {
            event: Event::new_inherited(),
            // info: init.info.clone(),
            destination: init.destination.clone(),
            navigation_type: init.navigationType.clone(),
            interception_state: DomRefCell::new(InterceptionState::None),
            navigation_handler_list: DomRefCell::new(vec![]),
            focus_reset: DomRefCell::new(None),
            scroll_behavior: DomRefCell::new(None),
            download_request: DomRefCell::new(init.downloadRequest.clone()),
            can_intercept: DomRefCell::new(init.canIntercept),
            user_initiated: DomRefCell::new(init.userInitiated),
            has_ua_visible_transitions: DomRefCell::new(init.hasUAVisualTransition),
            hash_change: DomRefCell::new(init.hashChange),
            form_data: None,
        }
    }

    fn new_with_proto(
        window: &Window,
        proto: Option<HandleObject>,
        type_: Atom,
        init: &RootedTraceableBox<NavigateEventInit>,
        can_gc: CanGc,
    ) -> DomRoot<NavigateEvent> {
        let ev = reflect_dom_object_with_proto(
            Box::new(NavigateEvent::new_inherited(init)),
            window,
            proto,
            can_gc,
        );

        {
            let event = ev.upcast::<Event>();
            let parent = &init.parent;

            event.init_event(type_, parent.bubbles, parent.cancelable);
        }

        ev
    }

    pub fn new(
        window: &Window,
        proto: Option<HandleObject>,
        type_: Atom,
        init: RootedTraceableBox<NavigateEventInit>,
        can_gc: CanGc,
    ) -> DomRoot<NavigateEvent> {
        NavigateEvent::new_with_proto(window, proto, type_, &init, can_gc)
    }

    /// <https://html.spec.whatwg.org/multipage/#navigateevent-perform-shared-checks>
    fn perform_shared_checks(&self) -> Fallible<()> {
        let global = self.global();
        let window = global.as_window();
        let document = window.Document();

        // Step 1. If event's relevant global object's associated Document is not fully active, then
        // throw an "InvalidStateError" DOMException.
        if !document.is_fully_active() {
            return Err(Error::InvalidState);
        }

        // Step 2. If event's isTrusted attribute was initialized to false, then throw a
        // "SecurityError" DOMException.
        if !self.IsTrusted() {
            return Err(Error::Security);
        }

        // Step 3. If event's canceled flag is set, then throw an "InvalidStateError" DOMException.
        if self.event.status() == EventStatus::Canceled {
            return Err(Error::InvalidState);
        }

        Ok(())
    }

    /// <https://html.spec.whatwg.org/multipage/#process-scroll-behavior>
    fn process_scroll_behavior(&self) {
        // Step 1. Assert: event's interception state is "committed".
        // debug_assert_eq!(self.interception_state(), InterceptionState::Committed);

        // Step 2. Set event's interception state to "scrolled".
        *self.interception_state.borrow_mut() = InterceptionState::Scrolled;

        let global = self.global();
        let window = global.as_window();
        let document = window.Document();

        // Step 3. If event's navigationType was initialized to "traverse" or "reload", then restore
        // scroll position data given event's relevant global object's navigable's active session
        // history entry.
        if matches!(
            self.navigation_type,
            NavigationType::Reload | NavigationType::Traverse
        ) {
            // Step 3.1. Set document's target element to null.
            document.set_target_element(None);

            // Step 3.2. Scroll to the beginning of the document for document.
            document.check_and_scroll_fragment("", CanGc::note());

            // Step 3.3. Return
            return;
        } else {
            // Step 4. Otherwise
            // Step 4.1. Let document be event's relevant global object's associated Document.
            // Assigned above

            // Step 4.2. If document's indicated part is null, then scroll to the beginning of the
            // document given document.
            // TODO which url are we operating on?
            // let indicated_part = document.find_fragment_node()

            // Step 4.3. Otherwise, scroll to the fragment given document.
            document.check_and_scroll_fragment("", CanGc::note());
        }
    }
}

impl NavigateEventMethods<crate::DomTypeHolder> for NavigateEvent {
    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-navigationtype>
    fn NavigationType(&self) -> NavigationType {
        self.navigation_type.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-destination>
    fn Destination(&self) -> DomRoot<NavigationDestination> {
        self.destination.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-canintercept>
    fn CanIntercept(&self) -> bool {
        self.can_intercept.borrow().clone()
    }

    /// True if this navigation was due to a user clicking on an a element, submitting a form
    /// element, or using the browser UI to navigate; false otherwise.
    ///
    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-userinitiated>
    fn UserInitiated(&self) -> bool {
        self.user_initiated.borrow().clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-hashchange>
    fn HashChange(&self) -> bool {
        self.hash_change.borrow().clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-formdata>
    fn GetFormData(&self) -> Option<DomRoot<FormData>> {
        self.form_data.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-downloadrequest>
    fn GetDownloadRequest(&self) -> Option<DOMString> {
        self.download_request.borrow().clone()
    }

    /// An arbitrary JavaScript value passed via one of the navigation API methods which initiated
    /// this navigation, or undefined if the navigation was initiated by the user or by a different
    /// API.
    ///
    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-info>
    fn Info(&self, _cx: JSContext, _retval: MutableHandleValue) {
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-hasuavisualtransition>
    fn HasUAVisualTransition(&self) -> bool {
        self.has_ua_visible_transitions.borrow().clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-intercept>
    fn Intercept(&self, options: &NavigationInterceptOptions) -> Fallible<()> {
        // Step 1. Perform shared checks given this.
        self.perform_shared_checks()?;

        // Step 2. If this's canIntercept attribute was initialized to false, then throw a
        // "SecurityError" DOMException.
        if !self.CanIntercept() {
            return Err(Error::Security);
        }

        // Step 3. If this's dispatch flag is unset, then throw an "InvalidStateError" DOMException.
        if !self.event.dispatching() {
            return Err(Error::InvalidState);
        }

        let interception_state = self.interception_state.clone();

        // Assert: this's interception state is either "none" or "intercepted".
        debug_assert!(matches!(
            *interception_state.borrow(),
            InterceptionState::None | InterceptionState::Intercepted
        ));

        // Step 5. Set this's interception state to "intercepted".
        *self.interception_state.borrow_mut() = InterceptionState::Intercepted;

        // Step 6. If options["handler"] exists, then append it to this's navigation handler list.
        if let Some(ref handler) = options.handler {
            self.navigation_handler_list
                .borrow_mut()
                .push(handler.clone());
        }

        // Step 7. If options["focusReset"] exists, then:
        if options.focusReset.is_some() {
            // Step 7.1. If this's focus reset behavior is not null, and it is not equal to
            // options["focusReset"], then the user agent may report a warning to the console
            // indicating that the focusReset option for a previous call to intercept() was
            // overridden by this new value, and the previous value will be ignored.
            // TODO

            // Step 7.2. Set this's focus reset behavior to options["focusReset"].
            *self.focus_reset.borrow_mut() = options.focusReset.clone();
        }

        // Step 8. If options["scroll"] exists, then:
        if let Some(ref scroll_behavior) = options.scroll {
            // Step 8.1. If this's scroll behavior is not null, and it is not equal to
            // options["scroll"], then the user agent may report a warning to the console indicating
            // that the scroll option for a previous call to intercept() was overridden by this new
            // value, and the previous value will be ignored.
            // TODO

            // Step 8.2. Set this's scroll behavior to options["scroll"].
            *self.scroll_behavior.borrow_mut() = Some(scroll_behavior.clone());
        }

        Ok(())
    }

    /// <https://html.spec.whatwg.org/multipage/#dom-navigateevent-scroll>
    fn Scroll(&self) -> Fallible<()> {
        // Step 1. Perform shared checks given this.
        self.perform_shared_checks()?;

        // Step 2. If this's interception state is not "committed", then throw an
        // "InvalidStateError" DOMException.
        if self.interception_state.borrow().clone() != InterceptionState::Committed {
            return Err(Error::InvalidState);
        }

        // Step 3. Process scroll behavior given this.
        self.process_scroll_behavior();

        Ok(())
    }

    /// <https://dom.spec.whatwg.org/#dom-event-istrusted>
    fn IsTrusted(&self) -> bool {
        self.upcast::<Event>().IsTrusted()
    }

    /// <https://html.spec.whatwg.org/multipage/#the-navigateevent-interface>
    fn Constructor(
        window: &Window,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        type_: DOMString,
        init: RootedTraceableBox<NavigateEventInit>,
    ) -> DomRoot<NavigateEvent> {
        NavigateEvent::new_with_proto(window, proto, Atom::from(type_), &init, can_gc)
    }
}
