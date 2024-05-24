/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use servo_atoms::Atom;
use servo_url::ServoUrl;

use crate::dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use crate::dom::bindings::codegen::Bindings::SecurityPolicyViolationEventBinding::{
    SecurityPolicyViolationEventDisposition, SecurityPolicyViolationEventInit,
    SecurityPolicyViolationEventMethods,
};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::event::Event;
use crate::dom::globalscope::GlobalScope;

#[dom_struct]
/// <https://w3c.github.io/webappsec-csp/#violation-events>
pub struct SecurityPolicyViolationEvent {
    event: Event,
    document_uri: USVString,
    blocked_uri: USVString,
    referrer: USVString,
    disposition: SecurityPolicyViolationEventDisposition,
    effective_directive: DOMString,
    violated_directive: DOMString,
    original_policy: DOMString,
    line_number: u32,
    column_number: u32,
    status_code: u16,
    sample: DOMString,
    source_file: USVString,
}

impl SecurityPolicyViolationEvent {
    fn new_inherited(
        document_uri: USVString,
        blocked_uri: USVString,
        referrer: USVString,
        disposition: SecurityPolicyViolationEventDisposition,
        effective_directive: DOMString,
        violated_directive: DOMString,
        original_policy: DOMString,
        line_number: u32,
        column_number: u32,
        status_code: u16,
        sample: DOMString,
        source_file: USVString,
    ) -> SecurityPolicyViolationEvent {
        SecurityPolicyViolationEvent {
            event: Event::new_inherited(),
            document_uri,
            blocked_uri,
            referrer,
            disposition,
            effective_directive,
            violated_directive,
            original_policy,
            line_number,
            column_number,
            status_code,
            sample,
            source_file
        }
    }

    // pub fn new(
    //     global: &GlobalScope,
    //     type_: Atom,
    //     bubbles: bool,
    //     cancelable: bool,
    // ) -> DomRoot<SecurityPolicyViolationEvent> {
    //     Self::new_with_proto(global, None, type_, bubbles, cancelable)
    // }

    fn new_with_proto(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        type_: Atom,
        bubbles: bool,
        cancelable: bool,
        init: &SecurityPolicyViolationEventInit,
    ) -> DomRoot<SecurityPolicyViolationEvent> {
        let ev = reflect_dom_object_with_proto(
            Box::new(SecurityPolicyViolationEvent::new_inherited(
                init.documentURI.clone(),
                init.blockedURI.clone(),
                init.referrer.clone(),
                init.disposition,
                init.effectiveDirective.clone(),
                init.violatedDirective.clone(),
                init.originalPolicy.clone(),
                init.lineNumber,
                init.columnNumber,
                init.statusCode,
                init.sample.clone(),
                init.sourceFile.clone(),
            )),
            global,
            proto,
        );
        {
            let event = ev.upcast::<Event>();
            event.init_event(type_, bubbles, cancelable);
        }
        ev
    }

    #[allow(non_snake_case)]
    pub fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        type_: DOMString,
        init: &SecurityPolicyViolationEventInit,
    ) -> DomRoot<SecurityPolicyViolationEvent> {
        SecurityPolicyViolationEvent::new_with_proto(
            global,
            proto,
            Atom::from(type_),
            init.parent.bubbles,
            init.parent.cancelable,
            init,
        )
    }

    /// <https://w3c.github.io/webappsec-csp/#strip-url-for-use-in-reports>
    fn strip_url_for_use_in_reports(&self, mut url: ServoUrl) -> String {
        // If url’s scheme is not an HTTP(S) scheme, then return url’s scheme.
        if url.is_secure_scheme() {
            return String::from(url.scheme());
        }

        let (_, _, _) = (
            url.set_fragment(None),
            url.set_username(""),
            url.set_password(None),
        );

        // 5: Return the result of executing the URL serializer on url.
        // https://url.spec.whatwg.org/#concept-url-serializer

        url.into_string()
    }

    fn parse_property(&self, property: &USVString) -> USVString {
        if let Ok(url) = ServoUrl::parse(property) {
            let stripped_url = self.strip_url_for_use_in_reports(url);

            return USVString::from(stripped_url);
        }

        property.clone()
    }
}

#[allow(non_snake_case)]
impl SecurityPolicyViolationEventMethods for SecurityPolicyViolationEvent {
    fn DocumentURI(&self) -> USVString {
        self.parse_property(&self.document_uri)
    }

    fn Referrer(&self) -> USVString {
        self.parse_property(&self.referrer)
    }

    /// <https://w3c.github.io/webappsec-csp/#obtain-violation-blocked-uri>
    fn BlockedURI(&self) -> USVString {
        self.parse_property(&self.blocked_uri)
    }

    /// <https://w3c.github.io/webappsec-csp/#violation-effective-directive>
    fn EffectiveDirective(&self) -> DOMString {
        self.effective_directive.clone()
    }

    fn ViolatedDirective(&self) -> DOMString {
        self.violated_directive.clone()
    }

    fn OriginalPolicy(&self) -> DOMString {
        self.original_policy.clone()
    }

    fn SourceFile(&self) -> USVString {
        self.source_file.clone()
    }

    fn Sample(&self) -> DOMString {
        self.sample.clone()
    }

    fn Disposition(&self) -> SecurityPolicyViolationEventDisposition {
        self.disposition
    }

    fn StatusCode(&self) -> u16 {
        self.status_code
    }

    fn LineNumber(&self) -> u32 {
        self.line_number
    }

    fn ColumnNumber(&self) -> u32 {
        self.column_number
    }

    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
