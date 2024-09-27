/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use content_security_policy as csp;
use dom_struct::dom_struct;
use js::rust::HandleObject;
use urlpattern::{UrlPattern, UrlPatternOptions};
use urlpattern::UrlPatternInit;

use crate::dom::bindings::codegen::Bindings::URLPatternBinding::{
    URLPatternInit, URLPatternMethods, URLPatternOptions, URLPatternResult,
};
use crate::dom::bindings::codegen::UnionTypes;
use crate::dom::bindings::error::{Error, ErrorResult, Fallible};
use crate::dom::bindings::reflector::{reflect_dom_object_with_proto, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::USVString;
use crate::dom::globalscope::GlobalScope;
use crate::script_runtime::CanGc;

/// <https://urlpattern.spec.whatwg.org/#urlpattern>
#[dom_struct]
pub struct URLPattern {
    reflector_: Reflector,
    #[no_trace]
    #[ignore_malloc_size_of = "Channels are hard"]
    pattern: UrlPattern,
}

impl From<&URLPatternOptions> for UrlPatternOptions {
    fn from(options: &URLPatternOptions) -> UrlPatternOptions {
        UrlPatternOptions {
            ignore_case: options.ignoreCase
        }
    }
}

impl TryFrom<URLPatternInit> for UrlPatternInit {
    type Error = url::ParseError;

    fn try_from(init: URLPatternInit) -> Result<UrlPatternInit, Self::Error> {
        let base_url = if init.baseURL.is_some() {
            let url = csp::Url::parse(&init.baseURL.unwrap().0)?;

            Some(url)
        } else {
            None
        };

        Ok(UrlPatternInit {
            protocol: init.protocol.map(|v| v.0),
            username: init.username.map(|v| v.0),
            password: init.password.map(|v| v.0),
            hostname: init.hostname.map(|v| v.0),
            port: init.port.map(|v| v.0),
            pathname: init.pathname.map(|v| v.0),
            search: init.search.map(|v| v.0),
            hash: init.hash.map(|v| v.0),
            base_url,
        })
    }
}

impl From<URLPatternInit> for UrlPatternInit {
    fn from(init: URLPatternInit) -> UrlPatternInit {
        UrlPatternInit {
            protocol: init.protocol.map(|v| v.0),
            username: init.username.map(|v| v.0),
            password: init.password.map(|v| v.0),
            hostname: init.hostname.map(|v| v.0),
            port: init.port.map(|v| v.0),
            pathname: init.pathname.map(|v| v.0),
            search: init.search.map(|v| v.0),
            hash: init.hash.map(|v| v.0),
            base_url: init.baseURL.map(|v| csp::Url::parse(&v.0).unwrap()),
        }
    }
}

#[allow(non_snake_case)]
impl URLPattern {
    fn new(init: UnionTypes::USVStringOrURLPatternInit, url: Option<USVString>, options: &URLPatternOptions) -> URLPattern {
        let init = match init {
            // Step 2: If input is a scalar value string then:
            UnionTypes::USVStringOrURLPatternInit::USVString(value) => {
                UrlPatternInit::parse_constructor_string::<regex::Regex>(&value.0, None)
            },
            // Step 3: Otherwise:
            UnionTypes::USVStringOrURLPatternInit::URLPatternInit(init) => {
                Ok(UrlPatternInit::from(init))
            },
        };

        URLPattern {
            reflector_: Reflector::new(),
            pattern: UrlPattern::parse(init, UrlPatternOptions::from(options)).unwrap()
        }
    }

    /// <https://urlpattern.spec.whatwg.org/#url-pattern-create>
    pub fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        init: UnionTypes::USVStringOrURLPatternInit,
        url: USVString,
        options: &URLPatternOptions,
    ) -> DomRoot<URLPattern> {
        reflect_dom_object_with_proto(Box::new(URLPattern::new(init, Some(url), options)), global, proto, can_gc)
    }

    /// <https://urlpattern.spec.whatwg.org/#url-pattern-create>
    pub fn Constructor_(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        init: UnionTypes::USVStringOrURLPatternInit,
        options: &URLPatternOptions,
    ) -> DomRoot<URLPattern> {
        reflect_dom_object_with_proto(Box::new(URLPattern::new(init, None, options)), global, proto, can_gc)
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
        false
    }

    fn Exec(
        &self,
        input: UnionTypes::USVStringOrURLPatternInit,
        base_url: Option<USVString>,
    ) -> Option<URLPatternResult> {
        None
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-protocol>
    fn Protocol(&self) -> USVString {
        USVString::from(String::from(self.pattern.protocol()))
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-username>
    fn Username(&self) -> USVString {
        USVString::from(String::from(self.pattern.username()))
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-password>
    fn Password(&self) -> USVString {
        USVString::from(String::from(self.pattern.password()))
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-hostname>
    fn Hostname(&self) -> USVString {
        USVString::from(String::from(self.pattern.hostname()))
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-port>
    fn Port(&self) -> USVString {
        USVString::from(String::from(self.pattern.port()))
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-pathname>
    fn Pathname(&self) -> USVString {
        USVString::from(String::from(self.pattern.pathname()))
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-search>
    fn Search(&self) -> USVString {
        USVString::from(String::from(self.pattern.search()))
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-hash>
    fn Hash(&self) -> USVString {
        USVString::from(String::from(self.pattern.hash()))
    }

    /// <https://urlpattern.spec.whatwg.org/#dom-urlpattern-hasregexpgroups>
    fn HasRegExpGroups(&self) -> bool {
        self.pattern.has_regexp_groups()
    }
}
