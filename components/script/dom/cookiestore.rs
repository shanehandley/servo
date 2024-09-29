/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;
use std::time::Duration;

use cookie::{Cookie, SameSite};
use dom_struct::dom_struct;
use js::rust::HandleObject;
use js::jsval::{NullValue, UndefinedValue};
use servo_url::{ImmutableOrigin, ServoUrl};

use crate::dom::bindings::codegen::Bindings::CookieStoreBinding::{
    CookieInit, CookieStoreDeleteOptions, CookieStoreGetOptions, CookieStoreMethods,
};
use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{DomRoot, Root};
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::eventtarget::EventTarget;
use crate::dom::globalscope::GlobalScope;
use crate::dom::promise::Promise;
use crate::dom::window::Window;
use crate::realms::{AlreadyInRealm, InRealm};
use crate::script_runtime::CanGc;

#[dom_struct]
pub struct CookieStore {
    event: EventTarget,
}

impl CookieStore {
    pub fn new(global: &GlobalScope) -> DomRoot<CookieStore> {
        reflect_dom_object(
            Box::new(CookieStore {
                event: EventTarget::new_inherited(),
            }),
            global,
        )
    }

    pub fn new_with_proto(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> DomRoot<CookieStore> {
        reflect_dom_object(
            Box::new(CookieStore {
                event: EventTarget::new_inherited(),
            }),
            global,
        )
    }
}

impl CookieStoreMethods for CookieStore {
    /// <https://wicg.github.io/cookie-store/#dom-cookiestore-get>
    fn Get(&self, name: USVString) -> Fallible<Rc<Promise>> {
        // Step 1: Let settings be this's relevant settings object.
        let global = self.global();

        // Step 2: Let origin be settings’s origin.
        let origin = global.origin();

        // Step 3: If origin is an opaque origin, then return a promise rejected with a
        // "SecurityError" DOMException.
        if let ImmutableOrigin::Opaque(_) = origin.immutable() {
            return Err(Error::Security);
        }

        // Step 4: Let url be settings’s creation URL.
        let uri = global.creation_url().as_ref();

        // Step 5: Let p be a new promise.
        let in_realm_proof = AlreadyInRealm::assert();
        let promise = Promise::new_in_current_realm(InRealm::Already(&in_realm_proof));

        // Step 3: If origin is an opaque origin, then return a promise rejected with a
        // "SecurityError" DOMException.
        if !origin.is_tuple() {
            promise.reject_error(Error::Security);

            return Ok(promise);
        }

        // Ste 4: Let url be settings’s creation URL.
        let url: Option<ServoUrl> = global.creation_url().clone();

        // Step 6: Run the following steps in parallel:
        // Step 6.1: Let list be the results of running query cookies with url and name.
        // https://wicg.github.io/cookie-store/#query-cookies

        // Step 6.2: If list is failure, then reject p with a TypeError and abort these steps.

        // Step 6.3: If list is empty, then resolve p with null.
        let list: Vec<String> = vec![];

        if list.is_empty() {
            promise.resolve_native(&NullValue());
        }

        return Ok(promise);

        // Step 6.4: Otherwise, resolve p with the first item of list.

        // Step 7: Return p.
    }

    /// <https://wicg.github.io/cookie-store/#dom-cookiestore-get-options>
    fn Get_(&self, options: &CookieStoreGetOptions) -> Fallible<Rc<Promise>> {
        // Step 1: Let settings be this's relevant settings object.
        let global = self.global();

        // Step 2: Let origin be settings’s origin.
        let origin = global.origin();

        // Step 5: Let p be a new promise.
        let in_realm_proof = AlreadyInRealm::assert();
        let promise = Promise::new_in_current_realm(InRealm::Already(&in_realm_proof));

        // Step 3: If origin is an opaque origin, then return a promise rejected with a
        // "SecurityError" DOMException.
        if !origin.is_tuple() {
            promise.reject_error(Error::Security);

            return Ok(promise);
        }

        // Step 5: If options is empty, then return a promise rejected with a TypeError.
        if options.name.is_none() && options.url.is_none() {
            promise.reject_error(Error::Type(String::from("settings are empty")));

            return Ok(promise);
        }

        // Step 6: If options["url"] is present, then run these steps:
        if let Some(ref url) = options.url {
            // Step 6.1: Let parsed be the result of parsing options["url"] with settings’s API base
            // URL.
            let base_url = global.api_base_url();
            let url_string = url.0.clone();

            let parsed_url = ServoUrl::parse(&url_string);

            if parsed_url.is_err() {
                promise.reject_error(Error::Type(String::from("failed to parse url")));

                return Ok(promise);
            }

            let parsed_url_with_base = match ServoUrl::parse_with_base(Some(&base_url), &url_string)
            {
                Ok(result) => result,
                _ => base_url,
            };

            // Step 6.2: If this's relevant global object is a Window object and parsed does not
            // equal url, then return a promise rejected with a TypeError.
            let window = Root::downcast::<Window>(global);

            if window.is_some() {
                if parsed_url.as_ref().unwrap() != &parsed_url_with_base {
                    promise.reject_error(Error::Type(String::from("url mismatch")));

                    return Ok(promise);
                }
            }

            // Step 6.3: If parsed’s origin and url’s origin are not the same origin, then return a
            // promise rejected with a TypeError.
            if parsed_url_with_base.origin() != parsed_url.as_ref().unwrap().origin() {
                promise.reject_error(Error::Type(String::from("origin mismatch")));

                return Ok(promise);
            }

            // step 6.4: Set url to parsed.

            // Step 7: let p be a new promise

            // Step 8: Run the following steps in parallel:
        }

        let list: Vec<String> = vec![];

        promise.resolve_native(&list);

        return Ok(promise);
    }

    /// <https://wicg.github.io/cookie-store/#dom-cookiestore-getall>
    fn GetAll(&self, name: USVString) -> Rc<Promise> {
        // Step 1: Let settings be this's relevant settings object.
        let global = self.global();

        // Step 2: Let origin be settings’s origin.
        let origin = global.origin();

        // Step 9: Let p be a new promise.
        // Note set earlier than in spec
        let in_realm_proof = AlreadyInRealm::assert();
        let promise = Promise::new_in_current_realm(InRealm::Already(&in_realm_proof));

        // Step 3: If origin is an opaque origin, then return a promise rejected with a
        // "SecurityError" DOMException.
        if !origin.is_tuple() {
            promise.reject_error(Error::Security);

            return promise;
        }

        // Step 4: Let url be settings’s creation URL.
        let url = global.creation_url();

        // Step 5: Let domain be null

        // Step 6: let path be "/"
        let path = "/";

        // Step 7 let sameSite be `strict`

        // Step 8: Let partitioned be false
        let partitioned = false;

        // Step 9...
        // Step 10: Run the following steps in parallel:
        // Step 10.1: Let r be the result of running set a cookie with url, name, value, domain,
        // path, sameSite, and partitioned.

        // https://wicg.github.io/cookie-store/#set-a-cookie

        // Step 10.2: If r is failure, then reject p with a TypeError and abort these steps.

        // Step 10.3: Resolve p with undefined.
        promise.resolve_native(&UndefinedValue());

        // Step 11: Return p
        promise
    }

    /// <https://wicg.github.io/cookie-store/#dom-cookiestore-getall-options>
    fn GetAll_(&self, options: &CookieStoreGetOptions) -> Rc<Promise> {
        // Step 5: Let p be a new promise.
        let in_realm_proof = AlreadyInRealm::assert();
        let promise = Promise::new_in_current_realm(InRealm::Already(&in_realm_proof));

        promise.reject_error(Error::Security);

        promise
    }

    /// <https://wicg.github.io/cookie-store/#dom-cookiestore-set>
    fn Set(&self, name: USVString, value: USVString) -> Rc<Promise> {
        // Step 1: Let settings be this's relevant settings object.
        let global = self.global();

        // Step 2: Let origin be settings’s origin.
        let origin = global.origin();

        // Step 9: Let p be a new promise.
        // Note set earlier than in spec
        let in_realm_proof = AlreadyInRealm::assert();
        let promise = Promise::new_in_current_realm(InRealm::Already(&in_realm_proof));

        // Step 3: If origin is an opaque origin, then return a promise rejected with a
        // "SecurityError" DOMException.
        if !origin.is_tuple() {
            promise.reject_error(Error::Security);

            return promise;
        }

        // Step 4: Let url be settings’s creation URL.
        let url = global.creation_url();

        // Step 5: Let domain be null

        // Step 6: let path be "/"
        let path = "/";

        // Step 7 let sameSite be `strict`

        // Step 8: Let partitioned be false
        let partitioned = false;

        // Step 9...
        // Step 10: Run the following steps in parallel:
        // Step 10.1: Let r be the result of running set a cookie with url, name, value, domain,
        // path, sameSite, and partitioned.

        // https://wicg.github.io/cookie-store/#set-a-cookie

        // Step 10.2: If r is failure, then reject p with a TypeError and abort these steps.

        // Step 10.3: Resolve p with undefined.
        promise.resolve_native(&UndefinedValue());

        // Step 11: Return p
        promise
    }

    fn Set_(&self, options: &CookieInit) -> Rc<Promise> {
        // Step 1: Let settings be this's relevant settings object.
        let global = self.global();

        // Step 2: Let origin be settings’s origin.
        let origin = global.origin();

        // Step 9: Let p be a new promise.
        // Note set earlier than in spec
        let in_realm_proof = AlreadyInRealm::assert();
        let promise = Promise::new_in_current_realm(InRealm::Already(&in_realm_proof));

        // Step 3: If origin is an opaque origin, then return a promise rejected with a
        // "SecurityError" DOMException.
        if !origin.is_tuple() {
            promise.reject_error(Error::Security);

            return promise;
        }

        // Step 4: Let url be settings’s creation URL.
        let url: Option<ServoUrl> = global.creation_url().clone();

        // Step 5: Let domain be null

        // Step 6: let path be "/"
        let path = "/";

        // Step 7 let sameSite be `strict`

        // Step 8: Let partitioned be false
        let partitioned = false;

        // Step 9...
        // Step 10: Run the following steps in parallel:
        // Step 10.1: Let r be the result of running set a cookie with url, name, value, domain,
        // path, sameSite, and partitioned.
        let result = set_a_cookie(
            url.as_ref().unwrap().to_owned(),
            options.name.0.clone(),
            options.value.0.clone(),
            None,
            None,
            None,
            None,
            false,
        );

        // https://wicg.github.io/cookie-store/#set-a-cookie

        // Step 10.2: If r is failure, then reject p with a TypeError and abort these steps.

        // Step 10.3: Resolve p with undefined.
        promise.resolve_native(&NullValue());

        // Step 11: Return p
        promise
    }

    /// <https://wicg.github.io/cookie-store/#dom-cookiestore-delete>
    fn Delete(&self, name: USVString) -> Rc<Promise> {
        // Step 1: Let settings be this's relevant settings object.
        let global = self.global();

        // Step 2: Let origin be settings’s origin.
        let origin = global.origin();

        // Step 5: Let p be a new promise.
        // Note set earlier than in spec
        let in_realm_proof = AlreadyInRealm::assert();
        let promise = Promise::new_in_current_realm(InRealm::Already(&in_realm_proof));

        // Step 3: If origin is an opaque origin, then return a promise rejected with a
        // "SecurityError" DOMException.
        if !origin.is_tuple() {
            promise.reject_error(Error::Security);

            return promise;
        }

        // Step 4: Let url be settings’s creation URL.
        let url = global.creation_url();

        promise.resolve_native(&NullValue());

        promise
    }

    /// <https://wicg.github.io/cookie-store/#dom-cookiestore-delete-options>
    fn Delete_(&self, options: &CookieStoreDeleteOptions) -> Rc<Promise> {
       // Step 1: Let settings be this's relevant settings object.
       let global = self.global();

       // Step 2: Let origin be settings’s origin.
       let origin = global.origin();

       // Step 5: Let p be a new promise.
       // Note set earlier than in spec
       let in_realm_proof = AlreadyInRealm::assert();
       let promise = Promise::new_in_current_realm(InRealm::Already(&in_realm_proof));

       // Step 3: If origin is an opaque origin, then return a promise rejected with a
       // "SecurityError" DOMException.
       if !origin.is_tuple() {
           promise.reject_error(Error::Security);

           return promise;
       }

       // Step 4: Let url be settings’s creation URL.
       let url = global.creation_url();

        promise.resolve_native(&UndefinedValue());

        promise
    }

    fn GetOnchange(&self) -> Option<Rc<EventHandlerNonNull>> {
        None
    }

    fn SetOnchange(&self, value: Option<Rc<EventHandlerNonNull>>) {}
}

/// <https://wicg.github.io/cookie-store/#query-cookies>
fn query_cookies(url: ServoUrl, name: Option<String>) -> Vec<String> {
    vec![]
}

/// <https://wicg.github.io/cookie-store/#set-a-cookie>
fn set_a_cookie(
    url: ServoUrl,
    name: String,
    value: String,
    expires: Option<Duration>,
    domain: Option<String>,
    path: Option<String>,
    same_site: Option<SameSite>,
    partitioned: bool,
) -> Result<(), String> {
    fn contains_disallowed_chars(s: &str) -> bool {
        s.chars()
            .any(|ch| (ch.is_control() && ch != '\t') || ch == ';' || ch == '\x7F')
    }

    // Step 1: If name or value contain U+003B (;), any C0 control character except U+0009 (the
    // horizontal tab character), or U+007F, then return failure.
    if contains_disallowed_chars(&name) || contains_disallowed_chars(&value) {
        return Err(String::from("invalid control character detected"));
    }

    // Step 2: If name’s length is 0 and value contains U+003D (=), then return failure.
    if name.is_empty() && value.contains('=') {
        return Err(String::from("invalid value"));
    }

    // Step 3: If name’s length is 0 and value’s length is 0, then return failure.
    if name.is_empty() && value.is_empty() {
        return Err(String::from("name and value are empty"));
    }

    // Step 4: Let encodedName be the result of UTF-8 encoding name.
    // Step 5: Let encodedValue be the result of UTF-8 encoding value.

    // Step 6: If the byte sequence length of encodedName plus the byte sequence length of
    // encodedValue is greater than the maximum name/value pair size, then return failure.
    // This is 4096 bytes: https://wicg.github.io/cookie-store/#cookie-maximum-name-value-pair-size
    if name.as_bytes().len() + value.as_bytes().len() > 4096 {
        return Err(String::from("maximum length exceeded"));
    }

    // Step 7: Let host be url’s host
    let host = url.host().ok_or(String::from("url has an invalid host"))?;

    // Step 8: Let attributes be a new list.
    let mut cookie = Cookie::new(name, value);

    // Step 9: If domain is not null, then run these steps:
    if let Some(encoded_domain) = domain {
        // Step 9.1: If domain starts with U+002E (.), then return failure.
        if encoded_domain.starts_with(".") {
            return Err(String::from("invalid url domain: must not start with (.)"));
        }

        // Step 9.2: If host does not equal domain and host does not end with U+002E (.) followed by
        // domain, then return failure.

        // Step 9.3: Let encodedDomain be the result of UTF-8 encoding domain.

        // Step 9.4: If the byte sequence length of encodedDomain is greater than the maximum
        // attribute value size, then return failure.
        // https://wicg.github.io/cookie-store/#cookie-maximum-attribute-value-size
        if encoded_domain.as_bytes().len() > 1024 {
            return Err(String::from("invalid domain: maximum length exceeded"));
        }

        // Step 9.5: Append `Domain`/encodedDomain to attributes.
    }

    // Step 10: If expires is given, then append `Expires`/expires (date serialized) to attributes.

    // Step 11: If path does not start with U+002F (/), then return failure.
    if let Some(mut path) = path {
        if !path.starts_with("/") {
            return Err(String::from("invalid path: must begin with (/)"));
        }

        // Step 12: If path does not end with U+002F (/), then append U+002F (/) to path.
        if !path.ends_with("/") {
            path.push_str("/");
        }

        // Step 13: Let encodedPath be the result of UTF-8 encoding path.
        // Step 14: If the byte sequence length of encodedPath is greater than the maximum attribute
        // value size, then return failure.
        if path.as_bytes().len() > 1024 {
            return Err(String::from("invalid path: maximum length exceeded"));
        }

        cookie.set_path(path);
    }

    // Step 15: Append `Path`/encodedPath to attributes.

    // Step 16: Append `Secure`/`` to attributes.

    // Step 17: Switch on sameSite:
    match same_site {
        Some(SameSite::None) => {}
        Some(SameSite::Strict) => {}
        Some(SameSite::Lax) => {},
        _ => {}
    }

    // Step 18: If partitioned is true, Append `Partitioned`/`` to attributes.

    // Step 19: Perform the steps defined in Cookies § Storage Model for when the user agent
    // "receives a cookie" with url as request-uri, encodedName as cookie-name, encodedValue as
    // cookie-value, and attributes as cookie-attribute-list.
    // https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis-14#name-storage-model
    {
        // Create a new cookie with name cookie-name, value cookie-value. Set the creation-time and
        // the last-access-time to the current date and time.
        // let mut cookie = Cookie::new(name, value);

        

    }

    // Step 20: Return success.
    Ok(())
}

/// <https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis-14#name-storage-model>
fn set_cookie() {}

/// <https://wicg.github.io/cookie-store/#delete-a-cookie>
fn delete_a_cookie(url: ServoUrl, name: String, domain: String, path: String, partitioned: bool) {
    // Step 1: If path is not null, then run these steps:
    if !path.is_empty() {
        // Step 1.1: If path does not start with U+002F (/), then return failure.

        // Step 1.2: If path does not end with U+002F (/), then append U+002F (/) to path.
    }

    // Step 2: Let expires be the earliest representable date represented as a timestamp.

    // Step 3: Let value be the empty string.

    // Step 4: Let sameSite be "strict".
    let same_site = SameSite::Strict;

    // Step 5: Return the results of running set a cookie with url, name, value, expires, domain,
    // path, sameSite, and partitioned.


}
