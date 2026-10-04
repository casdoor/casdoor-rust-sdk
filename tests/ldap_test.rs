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

use casdoor_rust_sdk::{get_current_time, Ldap};
use common::{random_name, test_client};

/// The whole life cycle of an LDAP server, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_ldap() {
    let client = test_client();
    let id = random_name("Ldap");

    // An LDAP server belongs to an organization, the synced users are added to it.
    let ldap = Ldap {
        id: id.clone(),
        created_time: get_current_time(),
        server_name: "Test LDAP Server".to_string(),
        host: "localhost".to_string(),
        port: 389,
        username: "cn=admin,dc=example,dc=com".to_string(),
        password: "password".to_string(),
        base_dn: "dc=example,dc=com".to_string(),
        ..Default::default()
    };
    assert!(client.add_ldap(&ldap).await.unwrap());

    // Get all objects, check if our added object is inside the list.
    let ldaps = client.get_ldaps().await.unwrap();
    assert!(ldaps.iter().any(|item| item.id == id));

    // Get the object.
    let mut ldap = client.get_ldap(&id).await.unwrap().unwrap();
    assert_eq!(ldap.owner, client.config.organization_name);

    // Update the object.
    let updated_server_name = "Updated LDAP Server";
    ldap.server_name = updated_server_name.to_string();
    client.update_ldap(&ldap).await.unwrap();

    // Validate the update.
    let ldap = client.get_ldap(&id).await.unwrap().unwrap();
    assert_eq!(ldap.server_name, updated_server_name);

    // Delete the object.
    assert!(client.delete_ldap(&ldap).await.unwrap());

    // Validate the deletion.
    assert!(client.get_ldap(&id).await.unwrap().is_none());
}

/// The organizations are owned by `admin`, not by the client's organization.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_get_organization_names() {
    let client = test_client();
    let organizations = client.get_organization_names().await.unwrap();
    assert!(organizations
        .iter()
        .any(|item| item.name == client.config.organization_name));
}
