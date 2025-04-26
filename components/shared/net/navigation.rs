/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use content_security_policy::sandboxing_directive::SandboxingFlagSet;
use serde::{Deserialize, Serialize};

use crate::policy_container::PolicyContainer;

/// <https://html.spec.whatwg.org/multipage#source-snapshot-params>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceSnapshotParams {
    has_transient_activation: bool,
    sandboxing_flags: SandboxingFlagSet,
    allows_downloading: bool,
    source_policy_container: PolicyContainer,
}

impl SourceSnapshotParams {
    pub fn snapshot(
        has_transient_activation: bool,
        sandboxing_flags: SandboxingFlagSet,
        allows_downloading: bool,
        source_policy_container: PolicyContainer,
    ) -> SourceSnapshotParams {
        SourceSnapshotParams {
            has_transient_activation,
            sandboxing_flags,
            allows_downloading,
            source_policy_container,
        }
    }

    pub fn sandboxing_flags(&self) -> SandboxingFlagSet {
        self.sandboxing_flags
    }
}
