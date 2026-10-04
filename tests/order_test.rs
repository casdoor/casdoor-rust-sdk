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

use casdoor_rust_sdk::{get_current_time, Order, Product, ProductInfo};
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

/// The whole life cycle of an order, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_order() {
    let client = test_client();
    let owner = client.config.organization_name.clone();
    let product_name = random_name("OrderProduct");
    let order_name = random_name("Order");

    let product = Product {
        price: 1.0,
        ..new_product(&owner, &product_name)
    };
    assert!(
        client.add_product(&product).await.unwrap(),
        "failed to add product"
    );

    // Add a new object.
    let order = Order {
        owner: owner.clone(),
        name: order_name.clone(),
        created_time: get_current_time(),
        display_name: order_name.clone(),
        products: vec![product_name.clone()],
        product_infos: vec![ProductInfo {
            owner: owner.clone(),
            name: product_name.clone(),
            display_name: product_name.clone(),
            price: 1.0,
            currency: "USD".to_string(),
            quantity: 1,
            ..Default::default()
        }],
        user: owner.clone(),
        price: 1.0,
        currency: "USD".to_string(),
        state: "Created".to_string(),
        ..Default::default()
    };
    assert!(
        client.add_order(&order).await.unwrap(),
        "failed to add order"
    );

    // Get all the objects, check if our added object is inside the list.
    let orders = client.get_orders().await.unwrap();
    assert!(orders.iter().any(|item| item.name == order_name));

    // Get the orders of the user.
    let user_orders = client.get_user_orders(&owner).await.unwrap();
    assert!(user_orders.iter().any(|item| item.name == order_name));

    // Get the object.
    let mut order = client.get_order(&order_name).await.unwrap().unwrap();
    assert_eq!(order.name, order_name);

    // Update the object.
    let updated_message = "Updated order message";
    order.message = updated_message.to_string();
    assert!(client.update_order(&order).await.unwrap());

    // Validate the update.
    let order = client.get_order(&order_name).await.unwrap().unwrap();
    assert_eq!(order.message, updated_message);

    // Cancel the order.
    assert!(client.cancel_order(&order_name).await.unwrap());

    // Delete the object.
    assert!(client.delete_order(&order).await.unwrap());
    assert!(client.get_order(&order_name).await.unwrap().is_none());

    assert!(client.delete_product(&product).await.unwrap());
    assert!(client.get_product(&product_name).await.unwrap().is_none());
}
