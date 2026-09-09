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
struct EmailForm<'a> {
    title: &'a str,
    content: &'a str,
    sender: &'a str,
    receivers: &'a [&'a str],
}

impl Client {
    /// Send an email with the default email provider of the organization.
    pub async fn send_email(
        &self,
        title: &str,
        content: &str,
        sender: &str,
        receivers: &[&str],
    ) -> Result<()> {
        self.do_send_email(title, content, sender, receivers, &[])
            .await
    }

    /// Send an email with the given email provider.
    pub async fn send_email_by_provider(
        &self,
        title: &str,
        content: &str,
        sender: &str,
        provider: &str,
        receivers: &[&str],
    ) -> Result<()> {
        self.do_send_email(title, content, sender, receivers, &[("provider", provider)])
            .await
    }

    async fn do_send_email(
        &self,
        title: &str,
        content: &str,
        sender: &str,
        receivers: &[&str],
        query: &[(&str, &str)],
    ) -> Result<()> {
        let post_bytes = serde_json::to_vec(&EmailForm {
            title,
            content,
            sender,
            receivers,
        })?;

        self.do_post("send-email", query, PostBody::Raw(post_bytes))
            .await?;

        Ok(())
    }
}
