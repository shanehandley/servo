/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Cell;
use std::convert::TryInto;

use dom_struct::dom_struct;
use js::jsval::JSVal;
use lazy_static::lazy_static;

use crate::dom::bindings::codegen::Bindings::NavigationActivationBinding::NavigationActivationMethods;
use crate::dom::bindings::codegen::Bindings::PerformanceNavigationTimingBinding::NavigationTimingType;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::bindings::utils::to_frozen_array;
use crate::dom::navigationhistoryentry::NavigationHistoryEntry;
use crate::dom::window::Window;
use crate::script_runtime::JSContext;

#[dom_struct]
pub struct NavigationActivation {
    reflector_: Reflector,
}

impl NavigationActivation {
    pub fn Constructor() -> NavigationActivation {
        NavigationActivation {
            reflector_: Reflector::new()
        }
    }
}

impl NavigationActivationMethods for NavigationActivation {
    fn GetFrom(&self) -> Option<DomRoot<NavigationHistoryEntry>> {
        None
    }

    fn Entry(&self) -> DomRoot<NavigationHistoryEntry> {
        todo!()
    }

    fn NavigationTimingType(&self) -> NavigationTimingType {
        todo!()
    }
}
