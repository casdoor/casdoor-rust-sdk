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

use casdoor_rust_sdk::{get_current_time, Invitation};
use common::{random_name, test_client};

/// The whole life cycle of an invitation, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_invitation() {
    let client = test_client();
    let name = random_name("unit_test_invitation");
    let code = "TEST1234";

    // Add a new object.
    let invitation = Invitation {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        created_time: get_current_time(),
        display_name: "Test Invitation".to_string(),
        code: code.to_string(),
        default_code: code.to_string(),
        quota: 10,
        used_count: 0,
        application: client.config.application_name.clone(),
        email: "test@example.com".to_string(),
        signup_group: "test-group".to_string(),
        state: "Active".to_string(),
        ..Default::default()
    };
    assert!(client.add_invitation(&invitation).await.unwrap());

    // Get the object.
    let mut invitation = client.get_invitation(&name).await.unwrap().unwrap();
    assert_eq!(invitation.code, code);

    // Get all the objects.
    let invitations = client.get_invitations().await.unwrap();
    assert!(!invitations.is_empty());

    // Update the object.
    invitation.state = "Suspended".to_string();
    assert!(client.update_invitation(&invitation).await.unwrap());

    // Make it active again so that it can be found by its code.
    invitation.state = "Active".to_string();
    assert!(client.update_invitation(&invitation).await.unwrap());

    // Get the object by its code.
    let invitation_info = client
        .get_invitation_info(code, "app-casbin")
        .await
        .unwrap();
    assert!(invitation_info.is_some());

    // Delete the object.
    assert!(client.delete_invitation(&invitation).await.unwrap());

    // Validate the deletion.
    assert!(!matches!(client.get_invitation(&name).await, Ok(Some(_))));
}
