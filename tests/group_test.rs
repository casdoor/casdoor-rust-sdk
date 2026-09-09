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

use casdoor_rust_sdk::Group;
use common::{random_name, test_client};

/// The whole life cycle of a group, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_group() {
    let client = test_client();
    let name = random_name("Group");

    // Add a new object.
    let group = Group {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        display_name: name.clone(),
        ..Default::default()
    };
    assert!(
        client.add_group(&group).await.unwrap(),
        "failed to add group"
    );

    // Get all the objects, check if our added object is inside the list.
    let groups = client.get_groups().await.unwrap();
    assert!(groups.iter().any(|group| group.name == name));

    // Get the object.
    let mut group = client.get_group(&name).await.unwrap().unwrap();
    assert_eq!(group.name, name);

    // Update the object.
    let updated_display_name = "Updated Display Name";
    group.display_name = updated_display_name.to_string();
    assert!(client.update_group(&group).await.unwrap());

    // Validate the update.
    let group = client.get_group(&name).await.unwrap().unwrap();
    assert_eq!(group.display_name, updated_display_name);

    // Delete the object.
    assert!(client.delete_group(&group).await.unwrap());

    // Validate the deletion.
    assert!(client.get_group(&name).await.unwrap().is_none());
}
