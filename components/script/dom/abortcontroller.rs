/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::{HandleObject, HandleValue};

use crate::dom::abortsignal::AbortSignal;
use crate::dom::bindings::codegen::Bindings::AbortControllerBinding::AbortControllerMethods;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object_with_proto};
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::globalscope::GlobalScope;
use crate::script_runtime::{CanGc, JSContext};

/// <https://dom.spec.whatwg.org/#abortcontroller>
#[dom_struct]
pub(crate) struct AbortController {
    reflector_: Reflector,
    signal: Dom<AbortSignal>,
}

impl AbortController {
    pub fn new(global: &GlobalScope) -> AbortController {
        let signal = AbortSignal::new(
            global, false,
        );

        AbortController {
            reflector_: Reflector::new(),
            signal: Dom::from_ref(&signal),
        }
    }
    
    fn new_inherited(signal: &AbortSignal) -> AbortController {
        AbortController {
            reflector_: Reflector::new(),
            signal: Dom::from_ref(signal),
        }
    }

    fn new_with_proto(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> DomRoot<AbortController> {
        reflect_dom_object_with_proto(
            Box::new(AbortController::new_inherited(&AbortSignal::new(
                global, false,
            ))),
            global,
            proto,
            can_gc,
        )
    }
}

impl AbortControllerMethods<crate::DomTypeHolder> for AbortController {
    /// <https://dom.spec.whatwg.org/#dom-abortcontroller-abortcontroller>
    fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> DomRoot<AbortController> {
        AbortController::new_with_proto(global, proto, can_gc)
    }

    /// <https://dom.spec.whatwg.org/#dom-abortcontroller-signal>
    fn Signal(&self) -> DomRoot<AbortSignal> {
        DomRoot::from_ref(&self.signal)
    }

    /// <https://dom.spec.whatwg.org/#dom-abortcontroller-abort>
    fn Abort(&self, _cx: JSContext, reason: HandleValue) {
        self.signal.signal_abort(reason);
    }
}
