/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::default::Default;

use dom_struct::dom_struct;
use js::rust::HandleObject;
use net_traits::blob_url_store::{get_blob_origin, parse_blob_url};
use net_traits::filemanager_thread::FileManagerThreadMsg;
use net_traits::{CoreResourceMsg, IpcSend};
use profile_traits::ipc;
use servo_url::ServoUrl;
use uuid::Uuid;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::URLPatternBinding::{
    URLPatternInit, URLPatternMethods, URLPatternOptions, URLPatternResult
};
use crate::dom::bindings::codegen::UnionTypes;
use crate::dom::bindings::error::{Error, ErrorResult, Fallible};
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, DomObject, Reflector};
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::blob::Blob;
use crate::dom::globalscope::GlobalScope;
use crate::dom::urlhelper::UrlHelper;
use crate::dom::urlsearchparams::URLSearchParams;

/// <https://urlpattern.spec.whatwg.org/#urlpattern>
#[dom_struct]
pub struct URLPattern {
    reflector_: Reflector,
}

#[allow(non_snake_case)]
impl URLPattern {
    fn new() -> URLPattern {
        URLPattern {
            reflector_: Reflector::new(),
        }
    }

    /// <https://urlpattern.spec.whatwg.org/#url-pattern-create>
    pub fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        init: UnionTypes::USVStringOrURLPatternInit,
        url: USVString,
        options: &URLPatternOptions
    ) -> DomRoot<URLPattern> {
        reflect_dom_object_with_proto(Box::new(URLPattern::new()), global, proto)
    }

    /// <https://urlpattern.spec.whatwg.org/#url-pattern-create>
    pub fn Constructor_(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        init: UnionTypes::USVStringOrURLPatternInit,
        options: &URLPatternOptions
    ) -> DomRoot<URLPattern> {
        reflect_dom_object_with_proto(Box::new(URLPattern::new()), global, proto)
    }

    /// <https://urlpattern.spec.whatwg.org/#parse-a-constructor-string>
    fn parse_constructor_string() -> Option<String> {
        None
    }

    /// https://urlpattern.spec.whatwg.org/#url-pattern-match
    fn perform_match(&self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
impl URLPatternMethods for URLPattern {
    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-test>
    fn Test(
        &self,
        input: UnionTypes::USVStringOrURLPatternInit,
        base_url: Option<USVString>,
    ) -> bool {
        // 1. Let result be the result of match given this's associated URL pattern, input, and
        // baseURL if given.
        self.perform_match()
    }

    fn Exec(
        &self,
        input: UnionTypes::USVStringOrURLPatternInit,
        base_url: Option<USVString>,
    ) -> Option<URLPatternResult> {
        todo!()
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-protocol>
    fn Protocol(&self) -> USVString {
        todo!()
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-username>
    fn Username(&self) -> USVString {
        todo!()
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-password>
    fn Password(&self) -> USVString {
        todo!()
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-hostname>
    fn Hostname(&self) -> USVString {
        todo!()
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-port>
    fn Port(&self) -> USVString {
        todo!()
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-pathname>
    fn Pathname(&self) -> USVString {
        todo!()
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-search>
    fn Search(&self) -> USVString {
        todo!()
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-hash>
    fn Hash(&self) -> USVString {
        todo!()
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-hasregexpgroups>
    fn HasRegExpGroups(&self) -> bool {
        todo!()
    }
}
