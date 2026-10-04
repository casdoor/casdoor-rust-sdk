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

use casdoor_rust_sdk::{get_current_time, Product, ProductInfo};
use common::{random_name, test_client};

fn new_product(owner: &str, name: &str) -> Product {
    Product {
        owner: owner.to_string(),
        name: name.to_string(),
        created_time: get_current_time(),
        display_name: name.to_string(),
        image: "https://cdn.casbin.org/img/casdoor-logo_1185x256.png".to_string(),
        description: "Casdoor Website".to_string(),
        tag: "auto_created_product_for_plan".to_string(),
        quantity: 999,
        sold: 0,
        state: "Published".to_string(),
        providers: vec!["provider_payment_dummy".to_string()],
        currency: "USD".to_string(),
        ..Default::default()
    }
}

/// The whole life cycle of a product, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_product() {
    let client = test_client();
    let name = random_name("Product");

    // Add a new object.
    let product = new_product(&client.config.organization_name, &name);
    assert!(
        client.add_product(&product).await.unwrap(),
        "failed to add product"
    );

    // Get all the objects, check if our added object is inside the list.
    let products = client.get_products().await.unwrap();
    assert!(products.iter().any(|item| item.name == name));

    // Get the object.
    let mut product = client.get_product(&name).await.unwrap().unwrap();
    assert_eq!(product.name, name);

    // Update the object.
    let updated_description = "Updated Casdoor Website";
    product.description = updated_description.to_string();
    assert!(client.update_product(&product).await.unwrap());

    // Validate the update.
    let product = client.get_product(&name).await.unwrap().unwrap();
    assert_eq!(product.description, updated_description);

    // Place an order of the product.
    let product_info = ProductInfo {
        name: name.clone(),
        quantity: 1,
        ..Default::default()
    };
    client.place_order(&[product_info], "admin").await.unwrap();

    // Delete the object.
    assert!(client.delete_product(&product).await.unwrap());

    // Validate the deletion.
    assert!(client.get_product(&name).await.unwrap().is_none());
}
