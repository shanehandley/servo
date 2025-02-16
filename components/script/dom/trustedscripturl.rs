/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::TrustedHTMLBinding::TrustedScriptURL_Binding::TrustedScriptURLMethods;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::globalscope::GlobalScope;
use crate::script_runtime::CanGc;

/// <https://w3c.github.io/trusted-types/dist/spec/#trustedscripturl>
#[dom_struct]
pub(crate) struct TrustedScriptURL {
    reflector_: Reflector,
    data: USVString,
}

impl TrustedScriptURL {
    pub fn new(global: &GlobalScope, data: USVString) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(Self {
                reflector_: Reflector::new(),
                data,
            }),
            global,
            CanGc::note(),
        )
    }
}

impl TrustedScriptURLMethods<crate::DomTypeHolder> for TrustedScriptURL {
    fn Stringifier(&self) -> DOMString {
        DOMString::from_string(self.data.clone().0)
    }

    fn ToJSON(&self) -> USVString {
        self.data.clone()
    }
}
