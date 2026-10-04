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

use casdoor_rust_sdk::{get_current_time, Permission};
use common::{random_name, test_client};

/// The whole life cycle of a permission, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_permission() {
    let client = test_client();
    let name = random_name("Permission");

    // Add a new object.
    let permission = Permission {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        created_time: get_current_time(),
        display_name: name.clone(),
        description: "Casdoor Website".to_string(),
        users: vec!["casbin/*".to_string()],
        model: "admin/user-model-built-in".to_string(),
        resource_type: "Application".to_string(),
        resources: vec!["app-casbin".to_string()],
        actions: vec!["Read".to_string(), "Write".to_string()],
        effect: "Allow".to_string(),
        is_enabled: true,
        ..Default::default()
    };
    assert!(
        client.add_permission(&permission).await.unwrap(),
        "failed to add permission"
    );

    // Get all the objects, check if our added object is inside the list.
    let items = client.get_permissions().await.unwrap();
    assert!(items.iter().any(|item| item.name == name));

    // Get the object.
    let mut permission = client.get_permission(&name).await.unwrap().unwrap();
    assert_eq!(permission.name, name);

    // Update the object.
    let updated_value = "Updated Casdoor Website";
    permission.description = updated_value.to_string();
    assert!(client.update_permission(&permission).await.unwrap());

    // Validate the update.
    let permission = client.get_permission(&name).await.unwrap().unwrap();
    assert_eq!(permission.description, updated_value);

    // Delete the object.
    assert!(client.delete_permission(&permission).await.unwrap());

    // Validate the deletion.
    assert!(client.get_permission(&name).await.unwrap().is_none());
}
