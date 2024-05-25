/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::CSPViolationReportBodyBinding::CSPViolationReportBodyMethods;
use crate::dom::bindings::codegen::Bindings::SecurityPolicyViolationEventBinding::SecurityPolicyViolationEventDisposition;
use crate::dom::bindings::str::{DOMString, USVString};
use crate::dom::reportbody::ReportBody;

/// <https://w3c.github.io/webappsec-csp/#cspviolationreportbody>
#[dom_struct]
pub struct CSPViolationReportBody {
    report_body: ReportBody,
}

impl CSPViolationReportBodyMethods for CSPViolationReportBody {
    fn DocumentURL(&self) -> USVString {
        USVString::from("".to_owned())
    }

    fn GetReferrer(&self) -> Option<USVString> {
        None
    }

    fn GetBlockedURL(&self) -> Option<USVString> {
        None
    }

    fn EffectiveDirective(&self) -> DOMString {
        DOMString::new()
    }

    fn OriginalPolicy(&self) -> DOMString {
        DOMString::new()
    }

    fn GetSourceFile(&self) -> Option<USVString> {
        None
    }

    fn GetSample(&self) -> Option<DOMString> {
        None
    }

    fn Disposition(&self) -> SecurityPolicyViolationEventDisposition {
        SecurityPolicyViolationEventDisposition::Report
    }

    fn StatusCode(&self) -> u16 {
        0
    }

    fn GetLineNumber(&self) -> Option<u32> {
        None
    }

    fn GetColumnNumber(&self) -> Option<u32> {
        None
    }
}
