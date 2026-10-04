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

use casdoor_rust_sdk::{get_current_time, Application};
use common::{random_name, test_client};

/// The whole life cycle of a application, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_application() {
    let client = test_client();
    let name = random_name("application");

    // Add a new object.
    let application = Application {
        owner: "admin".to_string(),
        name: name.clone(),
        created_time: get_current_time(),
        display_name: name.clone(),
        logo: "https://cdn.casbin.org/img/casdoor-logo_1185x256.png".to_string(),
        homepage_url: "https://casdoor.org".to_string(),
        description: "Casdoor Website".to_string(),
        organization: "casbin".to_string(),
        ..Default::default()
    };
    assert!(
        client.add_application(&application).await.unwrap(),
        "failed to add application"
    );

    // Get all the objects, check if our added object is inside the list.
    let items = client.get_applications().await.unwrap();
    assert!(items.iter().any(|item| item.name == name));

    // Get the object.
    let mut application = client.get_application(&name).await.unwrap().unwrap();
    assert_eq!(application.name, name);

    // Update the object.
    let updated_value = "Updated Casdoor Website";
    application.description = updated_value.to_string();
    assert!(client.update_application(&application).await.unwrap());

    // Validate the update.
    let application = client.get_application(&name).await.unwrap().unwrap();
    assert_eq!(application.description, updated_value);

    // Delete the object.
    assert!(client.delete_application(&application).await.unwrap());

    // Validate the deletion.
    assert!(client.get_application(&name).await.unwrap().is_none());
}
