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

use casdoor_rust_sdk::{get_current_time, Enforcer};
use common::{random_name, test_client};

/// The whole life cycle of a enforcer, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_enforcer() {
    let client = test_client();
    let name = random_name("Enforcer");

    // Add a new object.
    let enforcer = Enforcer {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        created_time: get_current_time(),
        display_name: name.clone(),
        model: "built-in/user-model-built-in".to_string(),
        adapter: "built-in/user-adapter-built-in".to_string(),
        description: "Casdoor Website".to_string(),
        ..Default::default()
    };
    assert!(
        client.add_enforcer(&enforcer).await.unwrap(),
        "failed to add enforcer"
    );

    // Get all the objects, check if our added object is inside the list.
    let items = client.get_enforcers().await.unwrap();
    assert!(items.iter().any(|item| item.name == name));

    // Get the object.
    let mut enforcer = client.get_enforcer(&name).await.unwrap().unwrap();
    assert_eq!(enforcer.name, name);

    // Update the object.
    let updated_value = "Updated Casdoor Website";
    enforcer.description = updated_value.to_string();
    assert!(client.update_enforcer(&enforcer).await.unwrap());

    // Validate the update.
    let enforcer = client.get_enforcer(&name).await.unwrap().unwrap();
    assert_eq!(enforcer.description, updated_value);

    // Delete the object.
    assert!(client.delete_enforcer(&enforcer).await.unwrap());

    // Validate the deletion.
    assert!(client.get_enforcer(&name).await.unwrap().is_none());
}
