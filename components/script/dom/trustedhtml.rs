/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::TrustedHTMLBinding::TrustedHTMLMethods;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use crate::script_runtime::CanGc;

/// <https://w3c.github.io/trusted-types/dist/spec/#trustedhtml>
#[dom_struct]
pub(crate) struct TrustedHTML {
    reflector_: Reflector,
    data: DOMString,
}

impl TrustedHTML {
    pub fn new(global: &GlobalScope, data: DOMString) -> DomRoot<Self> {
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

impl TrustedHTMLMethods<crate::DomTypeHolder> for TrustedHTML {
    fn Stringifier(&self) -> DOMString {
        self.data.clone()
    }

    fn ToJSON(&self) -> DOMString {
        self.data.clone()
    }
}
