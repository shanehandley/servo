/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// <https://w3c.github.io/clipboard-apis/#clipboardevent>
[Exposed=Window]
interface ClipboardEvent : Event {
  [Throws] constructor(DOMString type, optional ClipboardEventInit eventInitDict = {});

  readonly attribute DataTransfer? clipboardData;
};

// <https://w3c.github.io/clipboard-apis/#dictdef-clipboardeventinit>
dictionary ClipboardEventInit : EventInit {
  DataTransfer? clipboardData = null;
};
