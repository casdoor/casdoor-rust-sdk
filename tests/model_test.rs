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

use casdoor_rust_sdk::{get_current_time, Model};
use common::{random_name, test_client};

const MODEL_TEXT: &str = r#"[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act"#;

/// The whole life cycle of a model, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_model() {
    let client = test_client();
    let name = random_name("Model");

    // Add a new object.
    let model = Model {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        created_time: get_current_time(),
        display_name: name.clone(),
        model_text: MODEL_TEXT.to_string(),
        ..Default::default()
    };
    assert!(
        client.add_model(&model).await.unwrap(),
        "failed to add model"
    );

    // Get all the objects, check if our added object is inside the list.
    let items = client.get_models().await.unwrap();
    assert!(items.iter().any(|item| item.name == name));

    // Get the object.
    let mut model = client.get_model(&name).await.unwrap().unwrap();
    assert_eq!(model.name, name);

    // Update the object.
    let updated_value = "UpdatedName";
    model.display_name = updated_value.to_string();
    // The server doesn't report the update as affected, so only validate it below.
    client.update_model(&model).await.unwrap();

    // Validate the update.
    let model = client.get_model(&name).await.unwrap().unwrap();
    assert_eq!(model.display_name, updated_value);

    // Delete the object.
    assert!(client.delete_model(&model).await.unwrap());

    // Validate the deletion.
    assert!(client.get_model(&name).await.unwrap().is_none());
}
