/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// https://html.spec.whatwg.org/multipage/#navigationcurrententrychangeevent

[Exposed=Window]
interface NavigationCurrentEntryChangeEvent : Event {
  constructor(DOMString type, NavigationCurrentEntryChangeEventInit eventInitDict);

  readonly attribute NavigationType? navigationType;
  readonly attribute NavigationHistoryEntry from;
};

dictionary NavigationCurrentEntryChangeEventInit : EventInit {

//   NavigationType? navigationType = null;
  NavigationType navigationType;
  required NavigationHistoryEntry from;
};
