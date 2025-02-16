#![allow(unsafe_code)]

use std::collections::BTreeSet;
use std::result::Result::Err;

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use dom_struct::dom_struct;
use html5ever::{local_name, namespace_url, ns, QualName};
use js::rust::HandleValue;
use js::rust::wrappers::JS_ValueToSource;

use super::globalscope::GlobalScope;
use super::userscripts::load_script;
use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::TrustedHTMLBinding::TrustedTypePolicyFactory_Binding::TrustedTypePolicyFactoryMethods;
use crate::dom::bindings::codegen::Bindings::TrustedHTMLBinding::TrustedTypePolicyOptions;
use crate::dom::bindings::conversions::jsstring_to_str;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::import::module::jsapi;
use crate::dom::bindings::reflector::{DomGlobal, Reflector, reflect_dom_object};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::trustedhtml::TrustedHTML;
use crate::dom::trustedscript::TrustedScript;
use crate::dom::trustedtypepolicy::TrustedTypePolicy;
use crate::script_runtime::{CanGc, JSContext};

#[allow(unsafe_code)]
unsafe fn handle_value_to_string(cx: *mut jsapi::JSContext, value: HandleValue) -> DOMString {
    rooted!(in(cx) let mut js_string = std::ptr::null_mut::<jsapi::JSString>());

    match std::ptr::NonNull::new(JS_ValueToSource(cx, value)) {
        Some(js_str) => {
            js_string.set(js_str.as_ptr());
            jsstring_to_str(cx, js_str)
        },
        None => "<error converting value to string>".into(),
    }
}

/// <https://w3c.github.io/trusted-types/dist/spec/#trusted-type-policy-factory>
#[dom_struct]
pub(crate) struct TrustedTypePolicyFactory {
    reflector_: Reflector,
    /// <https://w3c.github.io/trusted-types/dist/spec/#trustedtypepolicyfactory-default-policy>
    default_policy: Option<DomRoot<TrustedTypePolicy>>,
    #[ignore_malloc_size_of = "todo"]
    /// <https://w3c.github.io/trusted-types/dist/spec/#trustedtypepolicyfactory-created-policy-names>
    created_policy_names: DomRefCell<BTreeSet<String>>,
}

impl TrustedTypePolicyFactory {
    pub fn new(global: &GlobalScope) -> DomRoot<TrustedTypePolicyFactory> {
        reflect_dom_object(
            Box::new(TrustedTypePolicyFactory {
                reflector_: Reflector::new(),
                default_policy: None,
                created_policy_names: DomRefCell::new(BTreeSet::new()),
            }),
            global,
            CanGc::note(),
        )
    }

    fn is_empty(&self, cx: JSContext, value: HandleValue) -> bool {
        let value = unsafe { handle_value_to_string(*cx, value) };

        value.str() == ""
    }
}

impl TrustedTypePolicyFactoryMethods<crate::DomTypeHolder> for TrustedTypePolicyFactory {
    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicyfactory-createpolicy>
    fn CreatePolicy(
        &self,
        policy_name: DOMString,
        options: &TrustedTypePolicyOptions,
    ) -> Fallible<DomRoot<TrustedTypePolicy>> {
        // Step 1. Let allowedByCSP be the result of executing Should Trusted Type policy creation
        // be blocked by Content Security Policy? algorithm with global, policyName and factory’s
        // created policy names value.

        // Step 2. If allowedByCSP is "Blocked", throw a TypeError and abort further steps.
        // TODO Implement this in Rust-CSP

        // Step 3. If policyName is default and the factory’s default policy value is not null,
        // throw a TypeError and abort further steps.
        if policy_name == DOMString::from_string(String::from("default")) &&
            self.default_policy.is_some()
        {
            return Err(Error::Type(String::from(
                "A default trusted type policy is already defined",
            )));
        }

        let global = &self.global();

        // Step 4. Let policy be a new TrustedTypePolicy object.
        // Step 5. Set policy’s name property value to policyName.
        // Step 6. Set policy’s options value to «[
        //  "createHTML" -> options["createHTML]",
        //  "createScript" -> options["createScript]",
        //  "createScriptURL" -> options["createScriptURL]"
        // ]».
        let policy = TrustedTypePolicy::new(
            global,
            policy_name.clone(),
            TrustedTypePolicyOptions {
                createHTML: options.createHTML.clone(),
                createScript: options.createScript.clone(),
                createScriptURL: options.createScriptURL.clone(),
            },
        );

        // Step 8. Append policyName to factory’s created policy names.
        self.created_policy_names
            .borrow_mut()
            .insert(String::from(policy_name.str()));

        // Step 9. Return policy.
        Ok(policy)
    }

    /// Returns true if value is an instance of TrustedHTML and has an associated data value set,
    /// false otherwise.
    ///
    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicyfactory-ishtml>
    fn IsHTML(&self, cx: JSContext, value: HandleValue) -> bool {
        self.is_empty(cx, value)
    }

    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicyfactory-isscript>
    fn IsScript(&self, cx: JSContext, value: HandleValue) -> bool {
        self.is_empty(cx, value)
    }

    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicyfactory-isscripturl>
    fn IsScriptURL(&self, cx: JSContext, value: HandleValue) -> bool {
        self.is_empty(cx, value)
    }

    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicyfactory-emptyhtml>
    fn EmptyHTML(&self) -> DomRoot<TrustedHTML> {
        TrustedHTML::new(&self.global(), DOMString::new())
    }

    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicyfactory-emptyscript>
    fn EmptyScript(&self) -> DomRoot<TrustedScript> {
        TrustedScript::new(&self.global(), DOMString::new())
    }

    /// Allows the authors to check if a Trusted Type is required for a given Element's property
    /// (IDL attribute).
    ///
    /// Example:
    ///
    /// ```js
    /// trustedTypes.getPropertyType('div', 'innerHTML'); // "TrustedHTML"
    /// trustedTypes.getPropertyType('foo', 'bar'); // null
    /// ```
    ///
    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicyfactory-getpropertytype>
    fn GetPropertyType(
        &self,
        tag_name: DOMString,
        property: DOMString,
        _element_namespace: Option<DOMString>,
    ) -> Option<DOMString> {
        // Step 1. Set localName to tagName in ASCII lowercase.
        let local_name = tag_name.to_ascii_lowercase();

        // Further parse this via local_name!
        match local_name.as_str() {
            "htmliframeelement" => {
                match property.to_ascii_lowercase().as_str() {
                    "srcdoc" => Some(DOMString::from_string(String::from("TrustedHTML"))),
                    _ => return None
                }
            },
            "htmlscriptelemnent" => {
                match property.to_ascii_lowercase().as_str() {
                    "innertext" => Some(DOMString::from_string(String::from("TrustedScript"))),
                    "src" => Some(DOMString::from_string(String::from("TrustedScriptURL"))),
                    "text" => Some(DOMString::from_string(String::from("TrustedScript"))),
                    "textcontent" => Some(DOMString::from_string(String::from("TrustedScript"))),
                    _ => return None
                }
            }
            _ => match property.to_ascii_lowercase().as_str() {
                "innerhtml" | "outerhtml" => Some(DOMString::from_string(String::from("TrustedHTML"))),
                _ => return None
            }
        }

        // Step 2. If elementNs is null or an empty string, set elementNs to HTML namespace.
        // let element_namespace = element_namespace.unwrap_or(ns!(html));
        // let element_namespace = ns!(html);

        // Step 3. Let interface be the element interface for localName and elementNs.
        // https://dom.spec.whatwg.org/#concept-element-interface

        // https://github.com/shanehandley/servo/blob/main/components/script/dom/create.rs#L281



        // let qual = QualName::new(None, element_namespace, local_name!(local_name.as_str()));

        // Some(DOMString::from_string(String::from("TrustedHTML")))
    }

    /// Example:
    ///
    /// ```js
    /// trustedTypes.getAttributeType('script', 'src'); // "TrustedScriptURL"
    /// trustedTypes.getAttributeType('foo', 'bar'); // null
    /// ```
    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicyfactory-getattributetype>
    fn GetAttributeType(
        &self,
        tag_name: DOMString,
        attribute: DOMString,
        element_namespace: Option<DOMString>,
        attribute_namespace: Option<DOMString>,
    ) -> Option<DOMString> {
        // Step 1. Set localName to tagName in ASCII lowercase.
        let local_name = tag_name.to_ascii_lowercase();

        // ...must the same as above

        None
    }

    fn GetDefaultPolicy(&self) -> Option<DomRoot<TrustedTypePolicy>> {
        self.default_policy.clone()
    }
}
