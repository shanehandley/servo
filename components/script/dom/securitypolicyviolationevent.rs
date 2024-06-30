/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use content_security_policy::{Destination, InlineCheckType};
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
            original_policy,
            line_number,
            column_number,
            status_code,
            sample,
            source_file,
        }
    }

    pub fn new(
        global: &GlobalScope,
        bubbles: bool,
        cancelable: bool,
        url: Option<ServoUrl>,
        destination: Destination,
        check_type: Option<InlineCheckType>,
    ) -> DomRoot<SecurityPolicyViolationEvent> {
        let mut init = SecurityPolicyViolationEventInit::empty();

        if let Some(_url) = url {
            init.documentURI = strip_url_for_use_in_reports(_url).into()
        } else {
            init.documentURI = USVString(String::from("inline"))
        };

        warn!(
            "Setting the effectiveDirective: check_type is: {:?}",
            check_type
        );

        init.effectiveDirective = match (check_type, destination) {
            (Some(InlineCheckType::ScriptAttribute | InlineCheckType::Script), _) => {
                DOMString::from("script-src-attr".to_owned())
            },
            (Some(InlineCheckType::StyleAttribute | InlineCheckType::Style), _) => {
                DOMString::from("style-src-attr".to_owned())
            },
            (None, Destination::Script) => DOMString::from("script-src-elem".to_owned()),
            (None, Destination::Style) => DOMString::from("style-src-elem".to_owned()),
            (None, Destination::Audio) => DOMString::from("media-src".to_owned()),
            _ => {
                warn!("unhandled destination: {:?}", destination);

                DOMString::from("todo".to_owned())
            },
        };

        init.blockedURI = init.documentURI.clone();

        Self::new_with_proto(
            global,
            None,
            Atom::from("securitypolicyviolation".to_owned()),
            bubbles,
            cancelable,
            &init,
        )
    }

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

    fn parse_property(&self, property: &USVString) -> USVString {
        if let Ok(url) = ServoUrl::parse(property) {
            let stripped_url = strip_url_for_use_in_reports(url);

            return USVString::from(stripped_url);
        }

        property.clone()
    }
}

#[allow(non_snake_case)]
impl SecurityPolicyViolationEventMethods for SecurityPolicyViolationEvent {
    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-documenturi>
    fn DocumentURI(&self) -> USVString {
        self.parse_property(&self.document_uri)
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-referrer>
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

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-violateddirective>
    fn ViolatedDirective(&self) -> DOMString {
        self.effective_directive.clone()
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-originalpolicy>
    fn OriginalPolicy(&self) -> DOMString {
        self.original_policy.clone()
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-sourcefile>
    fn SourceFile(&self) -> USVString {
        self.source_file.clone()
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-sample>
    fn Sample(&self) -> DOMString {
        self.sample.clone()
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-disposition>
    fn Disposition(&self) -> SecurityPolicyViolationEventDisposition {
        self.disposition
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-statuscode>
    fn StatusCode(&self) -> u16 {
        self.status_code
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-linenumber>
    fn LineNumber(&self) -> u32 {
        self.line_number
    }

    /// <https://w3c.github.io/webappsec-csp/#ref-for-dom-securitypolicyviolationevent-columnnumber>
    fn ColumnNumber(&self) -> u32 {
        self.column_number
    }

    /// <https://dom.spec.whatwg.org/#dom-event-istrusted>
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}

/// <https://w3c.github.io/webappsec-csp/#strip-url-for-use-in-reports>
fn strip_url_for_use_in_reports(mut url: ServoUrl) -> String {
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
