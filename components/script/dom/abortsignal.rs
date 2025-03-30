/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;
use js::conversions::ToJSValConvertible;
use js::jsapi::{ExceptionStackBehavior, Heap, JS_IsExceptionPending};
use js::jsval::{JSVal, UndefinedValue};
use js::rust::wrappers::JS_SetPendingException;
use js::rust::{HandleValue, MutableHandleValue};
use script_bindings::error::Fallible;
use script_bindings::script_runtime::{CanGc, JSContext};

use super::bindings::weakref::WeakRef;
use super::promise::Promise;
use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::AbortSignalBinding::AbortSignalMethods;
//  use crate::dom::bindings::codegen::Bindings::EventListenerBinding::EventListener;
//  use crate::dom::bindings::codegen::Bindings::EventTargetBinding::EventListenerOptions;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::{DomGlobal, reflect_dom_object};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::weakref::WeakReferenceable;
use crate::dom::domexception::DOMErrorName;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::eventtarget::EventTarget;
use crate::dom::globalscope::GlobalScope;
use crate::dom::types::DOMException;

/// <https://dom.spec.whatwg.org/#abortsignal-abort-algorithms>
#[derive(JSTraceable, MallocSizeOf)]
pub enum AbortAlgorithm {
    StreamAbort(#[ignore_malloc_size_of = "Rc"] Rc<Promise>),
    // A promise resolved with undefined
    ResolveUndefined(#[ignore_malloc_size_of = "Rc"] Rc<Promise>),
    /// <https://fetch.spec.whatwg.org/#abort-fetch>
    AbortFetch,
}

impl AbortAlgorithm {
    fn exec(self) {
        match self {
            Self::ResolveUndefined(promise) => {
                promise.resolve_native(&(), CanGc::note());
            },
            _ => {},
        }
    }
}

#[dom_struct]
pub struct AbortSignal {
    event_target: EventTarget,
    #[ignore_malloc_size_of = "Defined in rust-mozjs"]
    reason: Heap<JSVal>,
    abort_algorithms: DomRefCell<Vec<AbortAlgorithm>>,

    source_signals: DomRefCell<Vec<WeakRef<AbortSignal>>>,
    dependent_signals: DomRefCell<Vec<WeakRef<AbortSignal>>>,
    dependent: DomRefCell<bool>,
}

impl AbortSignal {
    pub fn new_inherited(dependent: bool) -> AbortSignal {
        Self {
            event_target: EventTarget::new_inherited(),
            reason: Heap::default(),
            abort_algorithms: DomRefCell::default(),
            source_signals: DomRefCell::default(),
            dependent_signals: DomRefCell::default(),
            dependent: DomRefCell::new(dependent),
        }
    }

    pub fn new(global: &GlobalScope, dependent: bool) -> DomRoot<AbortSignal> {
        warn!("creating new AbortSignal");

        reflect_dom_object(
            Box::new(Self::new_inherited(dependent)),
            global,
            CanGc::note(),
        )
    }

    /// <https://dom.spec.whatwg.org/#create-a-dependent-abort-signal>
    pub fn create_dependent_signal(
        global: &GlobalScope,
        signals: Vec<DomRoot<Self>>,
    ) -> DomRoot<Self> {
        // 1. Let resultSignal be a new object implementing signalInterface using realm.
        let result_signal = Self::new(global, false);

        // 2. For each signal of signals: if signal is aborted, then set resultSignal's abort
        // reason to signal's abort reason and return resultSignal.
        for signal in &signals {
            if signal.Aborted() {
                result_signal.reason.set(signal.reason.get().clone());

                return result_signal;
            }
        }

        // 3. Set resultSignal's dependent to true.
        result_signal.set_dependent(true);

        // 4. For each signal of signals:
        for signal in signals {
            // 4.1. If signal's dependent is false:
            if *signal.dependent.borrow() == false {
                // 4.1.1. Append signal to resultSignal’s source signals.
                result_signal
                    .source_signals
                    .borrow_mut()
                    .push(signal.downgrade());

                // 4.1.2. Append resultSignal to signal’s dependent signals.
                signal
                    .dependent_signals
                    .borrow_mut()
                    .push(WeakRef::new(&result_signal));
            } else {
                // 4.1. Otherwise, for each sourceSignal of signal’s source signals:
                for source_signal in signal.source_signals.borrow_mut().iter_mut() {
                    // ignore dropped source signals
                    if let Some(source_signal) = source_signal.root() {
                        // 4.2.1. Assert: sourceSignal is not aborted and not dependent.
                        assert!(!source_signal.Aborted());
                        assert!(!*source_signal.dependent.borrow());

                        // 4.2.2. Append sourceSignal to resultSignal’s source signals.
                        result_signal
                            .source_signals
                            .borrow_mut()
                            .push(WeakRef::new(&source_signal));

                        // 4.2.3. Append resultSignal to sourceSignal’s dependent signals.
                        (*source_signal)
                            .dependent_signals
                            .borrow_mut()
                            .push(WeakRef::new(&result_signal));
                    }
                }
            }
        }

        // Step 5. Return resultSignal.
        result_signal
    }

    /// <https://dom.spec.whatwg.org/#abortsignal-add>
    pub fn add_abort_algorithms(&self, alg: Vec<AbortAlgorithm>) {
        if !self.Aborted() {
            self.abort_algorithms.borrow_mut().extend(alg);
        }
    }

    /// Relates to <https://dom.spec.whatwg.org/#dom-abortsignal-timeout>
    ///
    /// <https://dom.spec.whatwg.org/#abortsignal-signal-abort>
    #[allow(unsafe_code)]
    pub fn signal_abort(&self, reason: HandleValue) {
        // Step 1. If signal is aborted, then return.
        if self.Aborted() {
            return;
        }

        // Step 2. Set signal’s abort reason to reason if it is given; otherwise to a new "AbortError"
        // DOMException.
        let cx = *GlobalScope::get_cx();
        rooted!(in(cx) let mut new_reason = UndefinedValue());
        let reason = if reason.is_undefined() {
            let exception =
                DOMException::new(&self.global(), DOMErrorName::AbortError, CanGc::note());
            unsafe {
                exception.to_jsval(cx, new_reason.handle_mut());
            };
            new_reason.handle()
        } else {
            reason
        };

        self.reason.set(reason.get());

        // Step 3. For each algorithm of signal’s abort algorithms: run algorithm.
        // Step 4. Empty signal’s abort algorithms.
        for algorithm in self.abort_algorithms.borrow_mut().drain(..) {
            algorithm.exec();
        }

        // Step 5. Fire an event named abort at signal.
        let event = Event::new(
            &self.global(),
            atom!("abort"),
            EventBubbles::DoesNotBubble,
            EventCancelable::Cancelable,
            CanGc::note(),
        );

        event.fire(self.upcast(), CanGc::note());

        // Step 6. For each dependentSignal of signal’s dependent signals, signal abort on
        // dependentSignal with signal’s abort reason.
        for dependent_signal in self.dependent_signals.borrow().iter() {
            if let Some(signal) = dependent_signal.clone().root() {
                signal.signal_abort(reason);
            }
        }
    }

    fn set_dependent(&self, value: bool) {
        *self.dependent.borrow_mut() = value;
    }
}

impl AbortSignalMethods<crate::DomTypeHolder> for AbortSignal {
    // <https://dom.spec.whatwg.org/#dom-abortsignal-onabort>
    event_handler!(Abort, GetOnabort, SetOnabort);

    /// <https://dom.spec.whatwg.org/#dom-abortsignal-abort>
    #[allow(unsafe_code)]
    fn Abort(_cx: JSContext, global: &GlobalScope, reason: HandleValue) -> DomRoot<AbortSignal> {
        // Step 1. Let signal be a new AbortSignal object.
        let signal = AbortSignal::new(global, false);

        // Step 2. Set signal's abort reason to reason if it is given; otherwise to a new
        // "AbortError" DOMException.
        let cx = *GlobalScope::get_cx();
        rooted!(in(cx) let mut new_reason = UndefinedValue());

        let reason = if reason.is_undefined() {
            let exception = DOMException::new(global, DOMErrorName::AbortError, CanGc::note());
            unsafe {
                exception.to_jsval(cx, new_reason.handle_mut());
            };
            new_reason.handle()
        } else {
            reason
        };

        signal.reason.set(reason.get());

        // Step 3: Return signal.
        signal
    }

    /// <https://dom.spec.whatwg.org/#dom-abortsignal-aborted>
    fn Aborted(&self) -> bool {
        !self.reason.get().is_undefined()
    }

    /// <https://dom.spec.whatwg.org/#dom-abortsignal-any>
    fn Any(global: &GlobalScope, signals: Vec<DomRoot<AbortSignal>>) -> DomRoot<AbortSignal> {
        AbortSignal::create_dependent_signal(global, signals)
    }

    /// <https://dom.spec.whatwg.org/#dom-abortsignal-reason>
    fn Reason(&self, _cx: JSContext, mut retval: MutableHandleValue) {
        retval.set(self.reason.get());
    }

    #[allow(unsafe_code)]
    /// <https://dom.spec.whatwg.org/#dom-abortsignal-throwifaborted>
    fn ThrowIfAborted(&self) -> Fallible<()> {
        let reason = self.reason.get();

        if !reason.is_undefined() {
            let cx = *GlobalScope::get_cx();
            unsafe {
                assert!(!JS_IsExceptionPending(cx));
                rooted!(in(cx) let mut thrown = UndefinedValue());
                reason.to_jsval(cx, thrown.handle_mut());
                JS_SetPendingException(cx, thrown.handle(), ExceptionStackBehavior::Capture);
            }
        }

        Ok(())
    }
}
