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
use crate::serde_util::null_to_default;

/// The name of the built-in Casdoor application.
pub const CASDOOR_APPLICATION: &str = "app-built-in";
/// The name of the built-in Casdoor organization.
pub const CASDOOR_ORGANIZATION: &str = "built-in";

/// Session has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/session.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Session {
    pub owner: String,
    pub name: String,
    pub application: String,
    pub created_time: String,

    #[serde(deserialize_with = "null_to_default")]
    pub session_id: Vec<String>,

    #[serde(rename = "ExclusiveSignin")]
    pub exclusive_signin: bool,
}

impl Client {
    /// Get the sessions of the client's organization.
    pub async fn get_sessions(&self) -> Result<Vec<Session>> {
        self.get_list("get-sessions", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the sessions of the client's organization, together with the total
    /// number of the sessions.
    pub async fn get_pagination_sessions(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Session>, i32)> {
        self.get_pagination(
            "get-sessions",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get the session of a user in an application.
    pub async fn get_session(&self, name: &str, application: &str) -> Result<Option<Session>> {
        let session_pk_id = format!("{}/{}", self.get_id(name), application);

        self.get_object("get-session", &[("sessionPkId", &session_pk_id)])
            .await
    }

    /// Add a session.
    pub async fn add_session(&self, session: &Session) -> Result<bool> {
        Ok(self.modify_session("add-session", session, &[]).await?.1)
    }

    /// Update a session.
    pub async fn update_session(&self, session: &Session) -> Result<bool> {
        Ok(self.modify_session("update-session", session, &[]).await?.1)
    }

    /// Update only the given columns of a session.
    pub async fn update_session_for_columns(
        &self,
        session: &Session,
        columns: &[&str],
    ) -> Result<bool> {
        Ok(self
            .modify_session("update-session", session, columns)
            .await?
            .1)
    }

    /// Delete a session.
    pub async fn delete_session(&self, session: &Session) -> Result<bool> {
        Ok(self.modify_session("delete-session", session, &[]).await?.1)
    }

    async fn modify_session(
        &self,
        action: &str,
        session: &Session,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut session = session.clone();
        session.owner = get_owner(&session.owner, &self.config.organization_name);

        let id = format!("{}/{}", session.owner, session.name);
        self.modify_object(action, &id, &session, columns).await
    }
}
