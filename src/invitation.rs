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

use serde::{Deserialize, Serialize};

use crate::client::{get_owner, Client, Response};
use crate::error::Result;

/// Invitation has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/invitation.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Invitation {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub display_name: String,

    pub code: String,
    pub is_regexp: bool,
    pub quota: i32,
    pub used_count: i32,

    pub application: String,
    pub username: String,
    pub email: String,
    pub phone: String,

    pub signup_group: String,
    pub default_code: String,

    pub state: String,
}

impl Invitation {
    /// Return the Casdoor object ID (`"owner/name"`) of the invitation.
    pub fn get_id(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

impl Client {
    /// Get the invitations of the client's organization.
    pub async fn get_invitations(&self) -> Result<Vec<Invitation>> {
        self.get_list(
            "get-invitations",
            &[("owner", &self.config.organization_name)],
        )
        .await
    }

    /// Get one page of the invitations of the client's organization, together with the
    /// total number of the invitations.
    pub async fn get_pagination_invitations(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Invitation>, i32)> {
        self.get_pagination(
            "get-invitations",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one invitation by name.
    pub async fn get_invitation(&self, name: &str) -> Result<Option<Invitation>> {
        self.get_object("get-invitation", &[("id", &self.get_id(name))])
            .await
    }

    /// Get the invitation of an invitation code in an application.
    pub async fn get_invitation_info(
        &self,
        code: &str,
        application_name: &str,
    ) -> Result<Option<Invitation>> {
        let application_id = format!("admin/{application_name}");

        self.get_object(
            "get-invitation-info",
            &[("applicationId", &application_id), ("code", code)],
        )
        .await
    }

    /// Add an invitation.
    pub async fn add_invitation(&self, invitation: &Invitation) -> Result<bool> {
        Ok(self
            .modify_invitation("add-invitation", invitation, &[])
            .await?
            .1)
    }

    /// Update an invitation.
    pub async fn update_invitation(&self, invitation: &Invitation) -> Result<bool> {
        Ok(self
            .modify_invitation("update-invitation", invitation, &[])
            .await?
            .1)
    }

    /// Update only the given columns of an invitation.
    pub async fn update_invitation_for_columns(
        &self,
        invitation: &Invitation,
        columns: &[&str],
    ) -> Result<bool> {
        Ok(self
            .modify_invitation("update-invitation", invitation, columns)
            .await?
            .1)
    }

    /// Delete an invitation.
    pub async fn delete_invitation(&self, invitation: &Invitation) -> Result<bool> {
        Ok(self
            .modify_invitation("delete-invitation", invitation, &[])
            .await?
            .1)
    }

    async fn modify_invitation(
        &self,
        action: &str,
        invitation: &Invitation,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut invitation = invitation.clone();
        invitation.owner = get_owner(&invitation.owner, &self.config.organization_name);

        let id = format!("{}/{}", invitation.owner, invitation.name);
        self.modify_object(action, &id, &invitation, columns).await
    }
}
