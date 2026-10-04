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

use casdoor_rust_sdk::Resource;
use common::test_client;

/// Upload, list, get and delete a resource, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_resource() {
    let client = test_client();
    let filename = "resource.rs";
    let data = std::fs::read(format!("src/{filename}")).unwrap();
    let name = format!("/casdoor/{filename}");
    let owner = client.config.organization_name.clone();

    // Upload a new object.
    client
        .upload_resource(&owner, &name, "", filename, &data)
        .await
        .unwrap();

    // Get all the objects, check if our added object is inside the list.
    let resources = client
        .get_resources(&owner, &owner, "", "", "", "")
        .await
        .unwrap();
    assert!(resources.iter().any(|item| item.tag == name));

    // Get the object.
    let resource: Resource = client
        .get_resource(&format!("{owner}/{name}"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(resource.tag, name);

    // Delete the object.
    assert!(client.delete_resource(&resource).await.unwrap());

    // Validate the deletion.
    assert!(client.get_resource(&name).await.unwrap().is_none());
}
