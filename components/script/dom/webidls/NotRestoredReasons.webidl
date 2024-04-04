/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
/*
 * The origin of this IDL file is
 * https://html.spec.whatwg.org/multipage/nav-history-apis.html#notrestoredreasons
 */

[Exposed=Window]
interface NotRestoredReasonDetails {
  readonly attribute DOMString reason;
  [Default] object toJSON();
};

[Exposed=Window]
interface NotRestoredReasons {
  readonly attribute DOMString? src;
  readonly attribute DOMString? id;
  readonly attribute DOMString? name;
  readonly attribute DOMString? url;
  readonly attribute /* FrozenArray<NotRestoredReasonDetails>? */ any reasons;
  readonly attribute /* FrozenArray<NotRestoredReasons>? */ any children; 
  [Default] object toJSON();
};
