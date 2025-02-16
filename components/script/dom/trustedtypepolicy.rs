/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use std::rc::Rc;

use dom_struct::dom_struct;
use js::rust::HandleValue;

use super::bindings::callback::ExceptionHandling;
use crate::dom::bindings::codegen::Bindings::TrustedHTMLBinding::TrustedTypePolicy_Binding::TrustedTypePolicyMethods;
use crate::dom::bindings::codegen::Bindings::TrustedHTMLBinding::{
    CreateHTMLCallback, CreateScriptCallback, CreateScriptURLCallback, TrustedTypePolicyOptions,
};
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::import::module::SafeJSContext;
use crate::dom::bindings::reflector::{DomGlobal, Reflector, reflect_dom_object};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::globalscope::GlobalScope;
use crate::dom::trustedhtml::TrustedHTML;
use crate::dom::trustedscript::TrustedScript;
use crate::dom::trustedscripturl::TrustedScriptURL;
use crate::script_runtime::CanGc;

pub enum TrustedTypeName {
    TrustedHTML,
    TrustedScript,
    TrustedScriptURL,
}

#[derive(PartialEq)]
enum TrustedResult {
    HTMLOrScript(DOMString),
    ScriptURL(USVString),
    Empty,
}

impl ToString for TrustedResult {
    fn to_string(&self) -> String {
        match self {
            TrustedResult::HTMLOrScript(data) => String::from(data.clone().str()),
            TrustedResult::ScriptURL(data) => String::from(data.clone().0),
            TrustedResult::Empty => String::new(),
        }
    }
}

/// <https://w3c.github.io/trusted-types/dist/spec/#trusted-type-policy>
#[dom_struct]
pub(crate) struct TrustedTypePolicy {
    reflector_: Reflector,
    name: DOMString,
    #[ignore_malloc_size_of = "todo"]
    options: TrustedTypePolicyOptions,
}

impl TrustedTypePolicy {
    pub fn new(
        global: &GlobalScope,
        name: DOMString,
        options: TrustedTypePolicyOptions,
    ) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(Self {
                reflector_: Reflector::new(),
                name,
                options,
            }),
            global,
            CanGc::note(),
        )
    }
}

impl TrustedTypePolicy {
    /// <https://w3c.github.io/trusted-types/dist/spec/#abstract-opdef-create-a-trusted-type>
    fn create_trusted_type(
        &self,
        name: TrustedTypeName,
        value: DOMString,
        can_gc: CanGc,
    ) -> Fallible<TrustedResult> {
        // Step 1. Let policyValue be the result of executing Get Trusted Type policy value with the
        // same arguments as this algorithm and additionally true as throwIfMissing.
        // Step 2. If the algorithm threw an error, rethrow the error and abort the following steps.
        let policy_value = self.get_trusted_type_policy_value(name, value, can_gc)?;

        // Step 3. Let dataString be the result of stringifying policyValue.
        // Step 4. If policyValue is null or undefined, set dataString to the empty string.

        // Step 5. Return a new instance of an interface with a type name trustedTypeName, with its
        // associated data value set to dataString.
        Ok(policy_value)
    }

    // <https://w3c.github.io/trusted-types/dist/spec/#abstract-opdef-get-trusted-type-policy-value>
    fn get_trusted_type_policy_value(
        &self,
        name: TrustedTypeName,
        input: DOMString,
        can_gc: CanGc,
    ) -> Fallible<TrustedResult> {
        // Step 1. Let functionName be a function name for the given trustedTypeName, based on the
        // following table:
        // Step 2. Let function be policy’s options[functionName].
        let policy_value_result = match name {
            TrustedTypeName::TrustedHTML => self.options.createHTML.clone().map(|callback| {
                if let Ok(r) = callback.Call__(input, vec![], ExceptionHandling::Report, can_gc) {
                    TrustedResult::HTMLOrScript(r.expect("Failed to extract result"))
                } else {
                    TrustedResult::Empty
                }
            }),
            TrustedTypeName::TrustedScript => self.options.createScript.clone().map(|callback| {
                if let Ok(r) = callback.Call__(input, vec![], ExceptionHandling::Report, can_gc) {
                    TrustedResult::HTMLOrScript(r.expect("Failed to extract result"))
                } else {
                    TrustedResult::Empty
                }
            }),
            TrustedTypeName::TrustedScriptURL => {
                self.options.createScriptURL.clone().map(|callback| {
                    if let Ok(r) = callback.Call__(input, vec![], ExceptionHandling::Report, can_gc)
                    {
                        TrustedResult::ScriptURL(r.expect("Failed to extract result"))
                    } else {
                        TrustedResult::Empty
                    }
                })
            },
        };

        // Step 3. If function is null, then:
        // Step 3.1. If throwIfMissing throw a TypeError.
        // Step 3.2. Else return null

        // Step 4. Let policyValue be the result of invoking function with value as a first
        // argument, items of arguments as subsequent arguments, and callback **this** value set to
        // null, rethrowing any exceptions.
        // let policy_value =
        let Some(result) = policy_value_result else {
            return Err(Error::Type("Failed to get trustred type polict".into()));
        };

        if result == TrustedResult::Empty {
            return Err(Error::Type("Empty policy returned".into()));
        }

        // Step 5. Return policyValue.
        return Ok(result);
    }
}

impl TrustedTypePolicyMethods<crate::DomTypeHolder> for TrustedTypePolicy {
    fn Name(&self) -> DOMString {
        self.name.clone()
    }

    /// Returns the result of executing the Create a Trusted Type algorithm, with the following
    /// arguments:
    ///
    /// policy: self
    /// trustedTypeName: "TrustedHTML"
    /// value: input
    /// arguments: arguments
    ///
    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicy-createhtml>
    fn CreateHTML(
        &self,
        _cx: SafeJSContext,
        input: DOMString,
        _arguments: Vec<HandleValue>,
        can_gc: CanGc,
    ) -> Fallible<DomRoot<TrustedHTML>> {
        let result = self.create_trusted_type(TrustedTypeName::TrustedHTML, input, can_gc);

        match result {
            Ok(TrustedResult::HTMLOrScript(data)) => Ok(TrustedHTML::new(&self.global(), data)),
            _ => Err(Error::Data),
        }
    }

    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicy-createscript>
    fn CreateScript(
        &self,
        _cx: SafeJSContext,
        input: DOMString,
        _arguments: Vec<HandleValue>,
        can_gc: CanGc,
    ) -> Fallible<DomRoot<TrustedScript>> {
        let result = self.create_trusted_type(TrustedTypeName::TrustedScript, input, can_gc);

        match result {
            Ok(TrustedResult::HTMLOrScript(data)) => Ok(TrustedScript::new(&self.global(), data)),
            _ => Err(Error::Data),
        }
    }

    /// <https://w3c.github.io/trusted-types/dist/spec/#dom-trustedtypepolicy-createscripturl>
    fn CreateScriptURL(
        &self,
        _cx: SafeJSContext,
        input: DOMString,
        _arguments: Vec<HandleValue>,
        can_gc: CanGc,
    ) -> Fallible<DomRoot<TrustedScriptURL>> {
        let result = self.create_trusted_type(TrustedTypeName::TrustedScriptURL, input, can_gc);

        match result {
            Ok(TrustedResult::ScriptURL(data)) => Ok(TrustedScriptURL::new(&self.global(), data)),
            _ => Err(Error::Data),
        }
    }
}
