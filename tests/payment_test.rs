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

use casdoor_rust_sdk::{get_current_time, Payment};
use common::{random_name, test_client};

/// The whole life cycle of a payment, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_payment() {
    let client = test_client();
    let name = random_name("Payment");

    // Add a new object.
    let payment = Payment {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        created_time: get_current_time(),
        display_name: name.clone(),
        products: vec!["casbin".to_string()],
        price: 10.0,
        currency: "USD".to_string(),
        ..Default::default()
    };
    assert!(client.add_payment(&payment).await.unwrap());

    // Get all the objects, check if our added object is inside the list.
    let payments = client.get_payments().await.unwrap();
    assert!(payments.iter().any(|item| item.name == name));

    // Get the object.
    let mut payment = client.get_payment(&name).await.unwrap().unwrap();
    assert_eq!(payment.name, name);

    // Update the object.
    let updated_products = vec!["casbin".to_string(), "casdoor".to_string()];
    payment.products = updated_products.clone();
    assert!(client.update_payment(&payment).await.unwrap());

    // Validate the update.
    let payment = client.get_payment(&name).await.unwrap().unwrap();
    assert_eq!(payment.products, updated_products);

    // Delete the object.
    assert!(client.delete_payment(&payment).await.unwrap());

    // Validate the deletion.
    assert!(client.get_payment(&name).await.unwrap().is_none());
}
