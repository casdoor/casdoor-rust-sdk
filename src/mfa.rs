// Copyright 2026 The Casdoor Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{Client, PostBody};
use crate::error::Result;
use crate::serde_util::null_to_default;

/// The MFA type sent by email.
pub const MFA_TYPE_EMAIL: &str = "email";
/// The MFA type sent by SMS.
pub const MFA_TYPE_SMS: &str = "sms";
/// The MFA type of an authenticator app (TOTP).
pub const MFA_TYPE_APP: &str = "app";

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MfaRequest {
    pub owner: String,
    pub mfa_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub secret: String,
    #[serde(rename = "recoveryCodes", skip_serializing_if = "String::is_empty")]
    pub recovery_code: String,
}

/// The data of the response of [`Client::initiate_mfa()`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MfaInitiateData {
    pub enabled: bool,
    pub is_preferred: bool,
    pub mfa_remember_in_hours: i32,
    pub mfa_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub recovery_codes: Vec<String>,
    pub secret: String,
    pub url: String,
}

impl Client {
    /// Start the setup of an MFA method for a user, the returned secret and URL are used
    /// to show the QR code of the authenticator app.
    pub async fn initiate_mfa(
        &self,
        owner: &str,
        mfa_type: &str,
        name: &str,
    ) -> Result<MfaInitiateData> {
        let request = MfaRequest {
            owner: owner.to_string(),
            mfa_type: mfa_type.to_string(),
            name: name.to_string(),
            ..Default::default()
        };

        let response = self
            .do_post("mfa/setup/initiate", &[], to_form(&request)?)
            .await?;

        if response.data.is_null() {
            return Ok(MfaInitiateData::default());
        }

        Ok(serde_json::from_value(response.data)?)
    }

    /// Verify the passcode of an MFA method that is being set up.
    pub async fn verify_mfa(
        &self,
        owner: &str,
        mfa_type: &str,
        name: &str,
        secret: &str,
        passcode: &str,
    ) -> Result<bool> {
        let fields = HashMap::from([
            ("owner".to_string(), owner.to_string()),
            ("mfaType".to_string(), mfa_type.to_string()),
            ("name".to_string(), name.to_string()),
            ("secret".to_string(), secret.to_string()),
            ("passcode".to_string(), passcode.to_string()),
        ]);

        let response = self
            .do_post("mfa/setup/verify", &[], PostBody::Form(fields))
            .await?;

        Ok(response.status == "ok")
    }

    /// Enable a verified MFA method for a user.
    pub async fn enable_mfa(
        &self,
        owner: &str,
        mfa_type: &str,
        name: &str,
        secret: &str,
        recovery_code: &str,
    ) -> Result<bool> {
        let request = MfaRequest {
            owner: owner.to_string(),
            mfa_type: mfa_type.to_string(),
            name: name.to_string(),
            secret: secret.to_string(),
            recovery_code: recovery_code.to_string(),
        };

        let response = self
            .do_post("mfa/setup/enable", &[], to_form(&request)?)
            .await?;

        Ok(response.status == "ok")
    }

    /// Set an MFA method as the preferred one of a user.
    pub async fn set_preferred_mfa(
        &self,
        owner: &str,
        mfa_type: &str,
        name: &str,
        secret: &str,
    ) -> Result<()> {
        let request = MfaRequest {
            owner: owner.to_string(),
            mfa_type: mfa_type.to_string(),
            name: name.to_string(),
            secret: secret.to_string(),
            ..Default::default()
        };

        self.do_post("set-preferred-mfa", &[], to_form(&request)?)
            .await?;

        Ok(())
    }

    /// Delete all the MFA methods of a user.
    pub async fn delete_mfa(&self, owner: &str, name: &str) -> Result<()> {
        self.do_post(
            "delete-mfa",
            &[("owner", owner), ("name", name)],
            PostBody::Raw(Vec::new()),
        )
        .await?;

        Ok(())
    }
}

/// The MFA APIs read their parameters from a form instead of from a JSON body.
fn to_form(request: &MfaRequest) -> Result<PostBody<'static>> {
    let fields = serde_json::from_value::<HashMap<String, String>>(serde_json::to_value(request)?)?;
    Ok(PostBody::Form(fields))
}
