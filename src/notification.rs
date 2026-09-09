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
struct NotificationForm<'a> {
    content: &'a str,
    #[serde(skip_serializing_if = "is_empty")]
    recipient: &'a str,
}

fn is_empty(value: &&str) -> bool {
    value.is_empty()
}

impl Client {
    /// Send a notification with the notification provider of the organization.
    /// `recipient` can be empty to use the default recipient of the provider.
    pub async fn send_notification(&self, content: &str, recipient: &str) -> Result<()> {
        let post_bytes = serde_json::to_vec(&NotificationForm { content, recipient })?;

        self.do_post("send-notification", &[], PostBody::Raw(post_bytes))
            .await?;

        Ok(())
    }
}
