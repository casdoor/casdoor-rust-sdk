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

use casdoor_rust_sdk::{get_current_time, Organization};
use common::{random_name, test_client};

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

/// The whole life cycle of an organization, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_organization() {
    let client = test_client();
    let name = random_name("Organization");

    // Add a new object.
    let organization = Organization {
        owner: "admin".to_string(),
        name: name.clone(),
        created_time: get_current_time(),
        display_name: name.clone(),
        website_url: "https://example.com".to_string(),
        password_type: "plain".to_string(),
        password_options: strings(&["AtLeast6"]),
        country_codes: strings(&[
            "US", "ES", "FR", "DE", "GB", "CN", "JP", "KR", "VN", "ID", "SG", "IN",
        ]),
        tags: vec![],
        languages: strings(&[
            "en", "zh", "es", "fr", "de", "id", "ja", "ko", "ru", "vi", "pt",
        ]),
        init_score: 2000,
        enable_soft_deletion: false,
        is_profile_public: false,
        ..Default::default()
    };
    assert!(client.add_organization(&organization).await.unwrap());

    // Get the object.
    let mut organization = client.get_organization(&name).await.unwrap().unwrap();
    assert_eq!(organization.name, name);

    // Update the object.
    let updated_display_name = "Updated Casdoor Website";
    organization.display_name = updated_display_name.to_string();
    assert!(client.update_organization(&organization).await.unwrap());

    // Validate the update.
    let organization = client.get_organization(&name).await.unwrap().unwrap();
    assert_eq!(organization.display_name, updated_display_name);

    // Delete the object.
    assert!(client.delete_organization(&organization).await.unwrap());

    // Validate the deletion.
    assert!(client.get_organization(&name).await.unwrap().is_none());
}
