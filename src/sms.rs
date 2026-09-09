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

use serde::Serialize;

use crate::client::{Client, PostBody};
use crate::error::Result;

#[derive(Serialize)]
struct SmsForm<'a> {
    content: &'a str,
    receivers: &'a [&'a str],
}

impl Client {
    /// Send an SMS with the default SMS provider of the organization.
    pub async fn send_sms(&self, content: &str, receivers: &[&str]) -> Result<()> {
        self.do_send_sms(content, receivers, &[]).await
    }

    /// Send an SMS with the given SMS provider.
    pub async fn send_sms_by_provider(
        &self,
        content: &str,
        provider: &str,
        receivers: &[&str],
    ) -> Result<()> {
        self.do_send_sms(content, receivers, &[("provider", provider)])
            .await
    }

    async fn do_send_sms(
        &self,
        content: &str,
        receivers: &[&str],
        query: &[(&str, &str)],
    ) -> Result<()> {
        let post_bytes = serde_json::to_vec(&SmsForm { content, receivers })?;

        self.do_post("send-sms", query, PostBody::Raw(post_bytes))
            .await?;

        Ok(())
    }
}
