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

mod common;

use casdoor_rust_sdk::{get_current_time, Session};
use common::{random_name, test_client};

/// The whole life cycle of a session, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_session() {
    let client = test_client();
    let name = random_name("Session");
    let application = "app-built-in";

    // Add a new object.
    let session = Session {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        created_time: get_current_time(),
        application: application.to_string(),
        session_id: vec![],
        ..Default::default()
    };
    assert!(client.add_session(&session).await.unwrap());

    // Get all the objects, check if our added object is inside the list.
    let sessions = client.get_sessions().await.unwrap();
    assert!(sessions.iter().any(|item| item.name == name));

    // Get the object.
    let mut session = client
        .get_session(&name, application)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(session.name, name);

    // Update the object.
    let updated_time = "2026-01-02T15:04:05Z";
    session.created_time = updated_time.to_string();
    assert!(client.update_session(&session).await.unwrap());

    // Validate the update.
    let session = client
        .get_session(&name, application)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(session.created_time, updated_time);

    // Delete the object.
    assert!(client.delete_session(&session).await.unwrap());

    // Validate the deletion.
    assert!(client
        .get_session(&name, application)
        .await
        .unwrap()
        .is_none());
}
