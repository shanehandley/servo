/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::NotRestoredReasonsBinding::NotRestoredReasonDetails_Binding::NotRestoredReasonDetailsMethods;
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;

const REASONS: &[&str] = &[
    "fetch",
    "navigation-failure",
    "parser-abortion",
    "websocket",
    "lock",
    "masked",
];

/// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#notrestoredreasondetails>
#[dom_struct]
pub struct NotRestoredReasonDetails {
    reflector_: Reflector,
    reason: DOMString,
}

impl NotRestoredReasonDetails {
    fn new_inherited(reason: DOMString) -> NotRestoredReasonDetails {
        NotRestoredReasonDetails {
            reflector_: Reflector::new(),
            reason,
        }
    }

    pub fn new(global: &GlobalScope, reason: DOMString) -> DomRoot<NotRestoredReasonDetails> {
        reflect_dom_object(
            Box::new(NotRestoredReasonDetails::new_inherited(reason)),
            global,
        )
    }
}

#[allow(non_snake_case)]
impl NotRestoredReasonDetailsMethods for NotRestoredReasonDetails {
    /// <https://html.spec.whatwg.org/multipage/nav-history-apis.html#dom-not-restored-reason-details-reason>
    fn Reason(&self) -> DOMString {
        self.reason.clone()
    }
}
