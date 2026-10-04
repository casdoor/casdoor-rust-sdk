# casdoor-rust-sdk

[![GitHub last commit](https://img.shields.io/github/last-commit/casdoor/casdoor-rust-sdk)](https://github.com/casdoor/casdoor-rust-sdk/commits/master)
[![Crates.io](https://img.shields.io/crates/v/casdoor-rust-sdk.svg)](https://crates.io/crates/casdoor-rust-sdk)
[![Docs](https://docs.rs/casdoor-rust-sdk/badge.svg)](https://docs.rs/casdoor-rust-sdk)
[![CI](https://github.com/casdoor/casdoor-rust-sdk/workflows/CI/badge.svg)](https://github.com/casdoor/casdoor-rust-sdk/actions)
[![Discord](https://img.shields.io/discord/1022748306096537660?logo=discord&label=discord&color=5865F2)](https://discord.gg/5rPsrAzK7S)

Casdoor Rust SDK is the official Rust client library for [Casdoor](https://casdoor.ai/). It lets you sign users in with Casdoor and manage users, organizations, applications, roles, permissions and every other Casdoor object from your Rust application.

The API follows the [Casdoor Go SDK](https://github.com/casdoor/casdoor-go-sdk): the same objects, the same actions, one Rust module per Go file, so the two SDKs stay easy to keep in sync.

## 📋 Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Configuration](#%EF%B8%8F-configuration)
- [Authentication](#-authentication)
- [Resource Management](#-resource-management)
- [API Reference](#-api-reference)
- [Examples](#-examples)
- [Migrating from 1.x](#-migrating-from-1x)
- [Development](#%EF%B8%8F-development)
- [Documentation](#-documentation)
- [License](#-license)

## ✨ Features

- **OAuth 2.0 Authentication**: the authorization code flow, token refresh, the password grant, impersonation and single sign-out
- **JWT Verification**: verify the tokens issued by Casdoor with the application's x509 certificate
- **User Management**: create, read, update and delete users, set passwords, look users up by email, phone or user ID
- **Organization Management**: manage organizations, applications, groups and invitations
- **Role-Based Access Control (RBAC)**: roles, permissions, models, adapters, enforcers, policies, `enforce()` and `batch_enforce()`
- **Resource Management**: certificates, providers, tokens, sessions, syncers, LDAP servers, webhooks, records and uploaded files
- **Multi-Factor Authentication (MFA)**: initiate, verify, enable, prefer and delete MFA methods
- **Email, SMS & Notifications**: send emails, SMS messages and notifications through the application's providers
- **Payment & Subscriptions**: products, orders, payments, transactions, plans, pricings and subscriptions
- **Async and Typed**: every API is `async` and returns `Result<T, CasdoorError>`, the objects are plain `serde` structs

## 📦 Installation

Add the SDK to your `Cargo.toml`. The APIs are `async`, so you also need an async runtime such as [Tokio](https://tokio.rs/):

```toml
[dependencies]
casdoor-rust-sdk = "2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## 🚀 Quick Start

```rust
use casdoor_rust_sdk::Client;

#[tokio::main]
async fn main() -> casdoor_rust_sdk::Result<()> {
    // Initialize the client with your Casdoor application's config
    let client = Client::new(
        "http://localhost:8000", // endpoint
        "CLIENT_ID",             // client_id
        "CLIENT_SECRET",         // client_secret
        "CERTIFICATE_CONTENT",   // certificate (x509 format)
        "my-organization",       // organization_name
        "my-application",        // application_name
    );

    // Get all users of the organization
    let users = client.get_users().await?;
    println!("Found {} users", users.len());

    Ok(())
}
```

## ⚙️ Configuration

### Creating a Client

A `Client` holds the config of one Casdoor application. Create as many clients as you need, for example one per Casdoor application or organization:

```rust
use casdoor_rust_sdk::Client;

// From the 6 config values
let client = Client::new(endpoint, client_id, client_secret, certificate, organization_name, application_name);

// From a TOML file, the path should be an absolute path (recommended)
let client = Client::from_toml("/path/to/casdoor.toml")?;

// A second client for another application
let client2 = Client::new(endpoint2, client_id2, client_secret2, certificate2, organization_name2, application_name2);
```

The TOML file looks like this, `org_name` and `app_name` are also accepted, so the config files of the 1.x versions keep working:

```toml
endpoint = "http://localhost:8000"
client_id = "<client-id>"
client_secret = "<client-secret>"
organization_name = "built-in"
application_name = "app-built-in"
certificate = """
-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----
"""
```

### Configuration Parameters

| Parameter         | Required | Description                                                     |
|-------------------|----------|-----------------------------------------------------------------|
| endpoint          | Yes      | Casdoor server URL (e.g., `http://localhost:8000`)              |
| client_id         | Yes      | Client ID of the Casdoor application                            |
| client_secret     | Yes      | Client secret of the Casdoor application                        |
| certificate       | Yes      | x509 certificate content of the application's cert (PEM format) |
| organization_name | Yes      | The name of the Casdoor organization                            |
| application_name  | No       | The name of the Casdoor application                             |

### Getting Configuration Parameters from Casdoor

1. **endpoint**: your Casdoor server URL
2. **client_id** and **client_secret**: found on your application's edit page in the Casdoor admin panel
3. **certificate**: the content of the cert used by your application, copied from the "Certs" page (x509 PEM format, a bare `PUBLIC KEY` PEM is also accepted)
4. **organization_name**: the organization that owns your application
5. **application_name**: your application's name in Casdoor

### Customizing the HTTP Client

The SDK sends requests with [reqwest](https://docs.rs/reqwest). Use `with_http_client()` to provide your own `reqwest::Client`, for example to set a timeout, a proxy or custom TLS options:

```rust
use std::time::Duration;

let http = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .pool_max_idle_per_host(10)
    // .proxy(reqwest::Proxy::all("http://proxy.example.com:8080")?)
    .build()?;

// Returns a copy of the client that uses the given HTTP client
let client = client.with_http_client(http);
```

### Custom HTTP Headers

Every request sent by a client includes the headers in its public `custom_headers` field. This is useful for:
- Getting localized error messages with `Accept-Language`
- Adding custom tenant identifiers
- Including trace IDs for debugging

```rust
// Returns a copy of the client with the extra header
let client = client.with_custom_header("Accept-Language", "de");

// Or modify the field directly
let mut client = client;
client.custom_headers.insert("X-Trace-ID".to_string(), "trace-abc-123".to_string());

// Now all API calls include these headers
let users = client.get_users().await?;
```

**Note**: the `Content-Type` and `Authorization` headers are managed by the SDK.

## 🔐 Authentication

### OAuth 2.0 Flow

#### Step 1: Redirect the User to Casdoor

```rust
// Casdoor sends the user back to the redirect URI with a `code` query parameter
let signin_url = client.get_signin_url("http://localhost:9000/callback");

// The sign-up page, `true` for the password sign-up page of the application
let signup_url = client.get_signup_url(true, "http://localhost:9000/callback");
```

#### Step 2: Handle the OAuth Callback

```rust
// Exchange the code for an access token
let token = client.get_oauth_token(&code).await?;

// Verify the token with the application's certificate and read the user out of it
let claims = client.parse_jwt_token(&token.access_token)?;
println!("signed in as {}", claims.user.name);
```

#### Step 3: Store the User Session

The claims are a `serde` struct, so they can be stored in your session as JSON:

```rust
let data = serde_json::to_string(&claims)?;
// Store in the session (depends on your web framework)
session.insert("user", data)?;
```

#### Sign Out

```rust
// Sign the user out of all their sessions (single sign-out)
client.logout(&token.access_token).await?;

// Or only out of the current session
client.logout_current_session(&token.access_token).await?;
```

### Token Refresh

```rust
let new_token = client.refresh_oauth_token(&token.refresh_token).await?;
```

The other ways to get a token are `get_oauth_token_by_password()` (the `password` grant type) and `impersonate_user()` (an admin acting as a user). `introspect_token()` checks whether a token is still active.

### Calling APIs With the User's Access Token

By default, the client calls the Casdoor APIs as the application itself, i.e., it authenticates with the client ID and client secret, so the calls have the application's (admin) permissions.

If you want to call the APIs on behalf of the signed-in user instead, use `with_access_token()` with the access token returned by `get_oauth_token()`. It returns a copy of the client that sends the `Authorization: Bearer <access_token>` header, so Casdoor treats the request as being made by that user, and the user's own permissions apply:

```rust
let token = client.get_oauth_token(&code).await?;

// A client that acts as the user who owns the access token
let user_client = client.with_access_token(&token.access_token);

// Get the user of the access token, i.e., "who am I"
let user = user_client.get_account().await?;

// Any other API can be called in the same way
let users = user_client.get_users().await?;
```

The original client is not affected, so it's safe to create one such client per incoming HTTP request.

**Note**: a non-admin user can only access their own data. If an API returns a permission error, the user simply isn't allowed to call it — use the application's client (without `with_access_token()`) for admin operations.

### JWT Token Parsing

```rust
let claims = client.parse_jwt_token(&access_token)?;

// Access the user information
println!("User: {}", claims.user.name);
println!("Email: {}", claims.user.email);
println!("Organization: {}", claims.user.owner);
```

### Profile Pages

```rust
// The profile page of a user, and of the signed-in user
let url = client.get_user_profile_url("alice", Some(&token.access_token));
let url = client.get_my_profile_url(Some(&token.access_token));
```

## 📦 Resource Management

All the API methods are `async` and return `Result<T, CasdoorError>`. The getters of a single object return `Option<T>`, which is `None` when the object doesn't exist, and `add_xxx()` / `update_xxx()` / `delete_xxx()` return whether the object was affected.

### Object Owner

Every object in Casdoor is identified by an ID of the form `owner/name`, where the owner is an organization (`role`, `group`, `user`, `product`, ...) or the built-in `admin` owner (`organization`, `application`, `token`, `ldap`).

By default the SDK fills in the owner for you: the `organization_name` of the client, or `admin` for the object types listed above. You can address an object in another organization by passing a qualified `owner/name` ID instead of a plain name, and by setting the `owner` field explicitly when creating or updating an object:

```rust
use casdoor_rust_sdk::Role;

// Uses the client's own organization: "my-org/my-role"
let role = client.get_role("my-role").await?;

// Uses the owner given in the name: "other-org/my-role"
let role = client.get_role("other-org/my-role").await?;

// Created in "other-org" instead of the client's organization
client
    .add_role(&Role {
        owner: "other-org".to_string(),
        name: "my-role".to_string(),
        ..Default::default()
    })
    .await?;
```

### User Management

```rust
use casdoor_rust_sdk::User;

// Get all users of the organization
let users = client.get_users().await?;

// Get a user by name, email, phone or user ID
let user = client.get_user("alice").await?;
let user = client.get_user_by_email("alice@example.com").await?;
let user = client.get_user_by_phone("+1234567890").await?;
let user = client.get_user_by_user_id("<user-id>").await?;

// Get the users page by page, with optional query filters
let (users, total_count) = client.get_pagination_users(1, 10, &[]).await?;

// Get the number of users, "1" for the online users and "" for all users
let count = client.get_user_count("").await?;

// Create a new user
let user = User {
    owner: "my-organization".to_string(),
    name: "alice".to_string(),
    display_name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
    password: "password123".to_string(),
    ..Default::default()
};
let affected = client.add_user(&user).await?;

// Update a user, or only some of its fields
let affected = client.update_user(&user).await?;
let affected = client.update_user_for_columns(&user, &["displayName", "email"]).await?;

// Delete a user
let affected = client.delete_user(&user).await?;

// Set or change the password of a user
let affected = client.set_password("my-organization", "alice", "old-password", "new-password").await?;

// Check the password of a user
let ok = client.check_user_password(&user).await?;
```

### Organization Management

```rust
use casdoor_rust_sdk::Organization;

let organizations = client.get_organizations().await?;
let organization = client.get_organization("my-organization").await?;
let names = client.get_organization_names().await?;

let organization = Organization {
    owner: "admin".to_string(),
    name: "new-org".to_string(),
    display_name: "New Organization".to_string(),
    ..Default::default()
};
client.add_organization(&organization).await?;
client.update_organization(&organization).await?;
client.delete_organization(&organization).await?;
```

### Application Management

```rust
let applications = client.get_applications().await?;
let applications = client.get_organization_applications().await?;
let application = client.get_application("my-application").await?;

client.add_application(&application).await?;
client.update_application(&application).await?;
client.delete_application(&application).await?;
```

### Role and Permission Management

```rust
use casdoor_rust_sdk::{Permission, Role};

let roles = client.get_roles().await?;
let role = client.get_role("admin").await?;
client
    .add_role(&Role {
        name: "admin".to_string(),
        display_name: "Administrator".to_string(),
        users: vec!["my-organization/alice".to_string()],
        is_enabled: true,
        ..Default::default()
    })
    .await?;

let permissions = client.get_permissions().await?;
let permissions = client.get_permissions_by_role("admin").await?;
client
    .add_permission(&Permission {
        name: "read-permission".to_string(),
        display_name: "Read Permission".to_string(),
        resources: vec!["app-my-application".to_string()],
        actions: vec!["Read".to_string()],
        effect: "Allow".to_string(),
        is_enabled: true,
        ..Default::default()
    })
    .await?;
```

### Permission Enforcement

`enforce()` checks a request against a permission, model, resource or enforcer, given by its `owner/name` ID. `batch_enforce()` checks several requests at once:

```rust
use serde_json::json;

let request = vec![json!("my-organization/alice"), json!("data1"), json!("read")];

// Check against a permission
let allowed = client.enforce("my-organization/read-permission", "", "", "", "", &request).await?;

// Check against an enforcer
let allowed = client.enforce("", "", "", "my-organization/my-enforcer", "", &request).await?;

// Check several requests at once
let results = client.batch_enforce("", "", "", "my-organization/my-enforcer", "", &[request]).await?;
```

### Models, Adapters, Enforcers and Policies

```rust
use casdoor_rust_sdk::{CasbinRule, PolicyFilter};

let models = client.get_models().await?;
let adapters = client.get_adapters().await?;
let enforcers = client.get_enforcers().await?;
let enforcer = client.get_enforcer("my-enforcer").await?.unwrap();

// Get, add, update and remove the policies of an enforcer
let policies = client.get_policies("my-enforcer", "").await?;
let policy = CasbinRule {
    ptype: "p".to_string(),
    v0: "alice".to_string(),
    v1: "data1".to_string(),
    v2: "read".to_string(),
    ..Default::default()
};
client.add_policy(&enforcer, &policy).await?;
client.update_policy(&enforcer, &policy, &new_policy).await?;
client.remove_policy(&enforcer, &new_policy).await?;

// Get the policies that match some filters
let filter = PolicyFilter {
    ptype: "p".to_string(),
    field_index: Some(0),
    field_values: vec!["alice".to_string()],
};
let policies = client.get_filtered_policies("my-organization/my-enforcer", &[filter]).await?;
```

### Session Management

```rust
let sessions = client.get_sessions().await?;
let session = client.get_session("alice", "my-application").await?;
client.delete_session(&session.unwrap()).await?;
```

### Provider, Certificate, Token and Webhook Management

```rust
let providers = client.get_providers().await?;
let provider = client.get_provider("provider_email_smtp").await?;

let certs = client.get_certs().await?;
let global_certs = client.get_global_certs().await?;
let cert = client.get_cert("cert-built-in").await?;

let tokens = client.get_tokens().await?;
let token = client.get_token("token-name").await?;

let webhooks = client.get_webhooks().await?;
let webhook = client.get_webhook("my-webhook").await?;
```

Each of them also has the `add_xxx()`, `update_xxx()` and `delete_xxx()` methods.

### Email, SMS and Notifications

```rust
// Send an email through the email provider of the application
client.send_email("Email Title", "Email content", "sender", &["alice@example.com"]).await?;

// Send an SMS message through the SMS provider of the application
client.send_sms("123456", &["+1234567890"]).await?;

// Send a notification through the notification provider of the application
client.send_notification("content", "").await?;
```

`send_email_by_provider()` and `send_sms_by_provider()` send the message through a given provider instead.

### Resource (File) Management

```rust
// Upload a file, returns its URL and the resource name
let (url, name) = client.upload_resource("alice", "avatar", "", "/avatar.png", &bytes).await?;

// Get the resources of a user
let resources = client.get_resources("my-organization", "alice", "", "", "", "").await?;

// Delete a resource
client.delete_resource(&resource).await?;
```

### Invitations

```rust
let invitations = client.get_invitations().await?;
let invitation = client.get_invitation("my-invitation").await?;

// Find an invitation by its code
let invitation = client.get_invitation_info("INVITATION_CODE", "my-application").await?;
```

### LDAP Servers

```rust
let ldaps = client.get_ldaps().await?;
let ldap = client.get_ldap("ldap-id").await?;

// Get the users of an LDAP server, and sync them to Casdoor
if let Some(response) = client.get_ldap_users("ldap-id").await? {
    client.sync_ldap_users("ldap-id", &response.users).await?;
}
```

### Products, Orders and Payments

```rust
use casdoor_rust_sdk::ProductInfo;

let products = client.get_products().await?;

// Place an order of some products for a user, then pay for it with a payment provider
let product_info = ProductInfo {
    name: "my-product".to_string(),
    quantity: 1,
    ..Default::default()
};
let order = client.place_order(&[product_info], "alice").await?;
let payment = client.pay_order(&order.name, "provider_payment_paypal").await?;

// Get the orders, payments and transactions of a user
let orders = client.get_user_orders("alice").await?;
let payments = client.get_user_payments("alice").await?;
let transactions = client.get_user_transactions("alice").await?;

// Cancel an order
client.cancel_order(&order.name).await?;
```

### Multi-Factor Authentication

```rust
// The arguments are the owner, the MFA type ("app", "sms" or "email") and the user name
let mfa = client.initiate_mfa("my-organization", "app", "alice").await?;
client.verify_mfa("my-organization", "app", "alice", &mfa.secret, "123456").await?;
client.enable_mfa("my-organization", "app", "alice", &mfa.secret, &mfa.recovery_codes[0]).await?;
client.set_preferred_mfa("my-organization", "app", "alice", &mfa.secret).await?;
client.delete_mfa("my-organization", "alice").await?;
```

## 📚 API Reference

### Available Resources

| Resource         | Description                                   | Key Operations                         |
|------------------|-----------------------------------------------|----------------------------------------|
| **User**         | User accounts and profiles                    | CRUD, password, lookup, pagination     |
| **Organization** | Organization entities                         | CRUD, names                            |
| **Application**  | Application configurations                    | CRUD                                   |
| **Group**        | User groups                                   | CRUD                                   |
| **Role**         | User roles                                    | CRUD                                   |
| **Permission**   | Access permissions                            | CRUD, enforcement, by role             |
| **Model**        | Casbin models                                 | CRUD                                   |
| **Adapter**      | Casbin policy adapters                        | CRUD                                   |
| **Enforcer**     | Casbin enforcers                              | CRUD, enforcement                      |
| **Policy**       | Casbin policies of an enforcer                | Get, filter, add, update, remove       |
| **Provider**     | Third-party providers (OAuth, email, SMS, ...)| CRUD                                   |
| **Cert**         | Certificates                                  | CRUD, global certs                     |
| **Token**        | Access and refresh tokens                     | CRUD, introspection                    |
| **Session**      | User sessions                                 | CRUD                                   |
| **Syncer**       | User synchronization from external databases  | CRUD                                   |
| **LDAP**         | LDAP servers                                  | CRUD, get and sync users               |
| **Webhook**      | Event webhooks                                | CRUD                                   |
| **Invitation**   | User invitations                              | CRUD, lookup by code                   |
| **Resource**     | Uploaded files                                | Upload, CRUD                           |
| **Record**       | Audit records                                 | Get, add                               |
| **Product**      | Products/services                             | CRUD, buy                              |
| **Order**        | Product orders                                | CRUD, place, pay, cancel               |
| **Payment**      | Payment records                               | CRUD, notify, invoice                  |
| **Transaction**  | Payment transactions                          | CRUD                                   |
| **Plan**         | Subscription plans                            | CRUD                                   |
| **Pricing**      | Pricing configurations                        | CRUD                                   |
| **Subscription** | User subscriptions                            | CRUD                                   |
| **MFA**          | Multi-factor authentication of a user         | Initiate, verify, enable, delete       |

### Method Patterns

Most resources follow the same method patterns:

- `get_xxxs()` - get all the objects of the organization
- `get_pagination_xxxs(page, page_size, query)` - get the objects page by page, with the total count
- `get_xxx(name)` - get one object by its name or `owner/name` ID, `None` if it doesn't exist
- `add_xxx(&object)` - create an object
- `update_xxx(&object)` - update an object
- `update_xxx_for_columns(&object, &columns)` - update only some fields of an object
- `delete_xxx(&object)` - delete an object

The full API documentation is on [docs.rs](https://docs.rs/casdoor-rust-sdk).

## 💡 Examples

### Complete Authentication Example

With [axum](https://github.com/tokio-rs/axum):

```rust
use std::collections::HashMap;

use axum::{extract::Query, response::Redirect, routing::get, Json, Router};
use casdoor_rust_sdk::{Claims, Client};

fn casdoor() -> Client {
    Client::new("http://localhost:8000", "CLIENT_ID", "CLIENT_SECRET", "CERTIFICATE", "my-organization", "my-application")
}

async fn login() -> Redirect {
    Redirect::to(&casdoor().get_signin_url("http://localhost:8080/callback"))
}

async fn callback(Query(query): Query<HashMap<String, String>>) -> Result<Json<Claims>, String> {
    let client = casdoor();
    let code = query.get("code").ok_or("missing code")?;

    let token = client.get_oauth_token(code).await.map_err(|err| err.to_string())?;
    let claims = client.parse_jwt_token(&token.access_token).map_err(|err| err.to_string())?;

    // Store the claims in the session, here they are simply returned
    Ok(Json(claims))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/login", get(login)).route("/callback", get(callback));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### User Management Example

```rust
use casdoor_rust_sdk::{Client, User};

#[tokio::main]
async fn main() -> casdoor_rust_sdk::Result<()> {
    let client = Client::new("http://localhost:8000", "CLIENT_ID", "CLIENT_SECRET", "CERT", "my-organization", "my-application");

    // Create a new user
    let user = User {
        name: "john_doe".to_string(),
        display_name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "secure_password".to_string(),
        ..Default::default()
    };
    let affected = client.add_user(&user).await?;
    println!("User created: {affected}");

    // Get and update the user
    if let Some(mut user) = client.get_user("john_doe").await? {
        user.display_name = "John D.".to_string();
        let affected = client.update_user(&user).await?;
        println!("User updated: {affected}");
    }

    Ok(())
}
```

### Permission Enforcement Example

```rust
use casdoor_rust_sdk::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> casdoor_rust_sdk::Result<()> {
    let client = Client::new("http://localhost:8000", "CLIENT_ID", "CLIENT_SECRET", "CERT", "my-organization", "my-application");

    // Check whether the user can read the resource under the permission
    let request = vec![json!("my-organization/user123"), json!("resource1"), json!("read")];
    let allowed = client.enforce("my-organization/my-permission", "", "", "", "", &request).await?;

    if allowed {
        println!("Access granted");
    } else {
        println!("Access denied");
    }

    Ok(())
}
```

## 🔄 Migrating from 1.x

The 2.0 version replaces `AuthService` and `UserService` with a single `Client`, which covers all the Casdoor objects instead of only the users, and makes every API return a typed `CasdoorError` instead of a `Box<dyn Error>`.

| 1.x                                     | 2.0                                           |
| --------------------------------------- | --------------------------------------------- |
| `CasdoorConfig::new(...)`               | `Client::new(...)`                            |
| `CasdoorConfig::from_toml(path)`        | `Client::from_toml(path)`                     |
| `CasdoorUser`                           | `User`                                        |
| `AuthService::new(&conf)`               | the `Client` itself                           |
| `auth.get_auth_token(code)`             | `client.get_oauth_token(&code).await?`        |
| `auth.parse_jwt_token(token)`           | `client.parse_jwt_token(&token)?`             |
| `auth.get_signin_url(uri)`              | `client.get_signin_url(&uri)`                 |
| `auth.get_signup_url_enable_password()` | `client.get_signup_url(true, "")`             |
| `auth.get_user_profile_url(name, tok)`  | `client.get_user_profile_url(&name, tok)`     |
| `UserService::new(&conf)`               | the `Client` itself                           |
| `user_service.get_user(name)`           | `client.get_user(&name).await?` (an `Option`) |
| `user_service.get_user_with_email(...)` | `client.get_user_by_email(&email).await?`     |
| `user_service.add_user(user)`           | `client.add_user(&user).await?` (a `bool`)    |

The certificate is now parsed as a real x509 certificate, so the certificate copied from Casdoor's "Certs" page works as-is, and a bare `PUBLIC KEY` PEM is still accepted.

## 🛠️ Development

```shell
cargo test
```

The tests that need a running Casdoor server are marked as ignored, they mirror the tests of the Casdoor Go SDK. Run them with:

```shell
cargo test -- --ignored
```

They read the server config from the `CASDOOR_TEST_ENDPOINT`, `CASDOOR_TEST_CLIENT_ID`, `CASDOOR_TEST_CLIENT_SECRET`, `CASDOOR_TEST_ORGANIZATION` and `CASDOOR_TEST_APPLICATION` environment variables. By default they connect to `http://localhost:8000`, which CI starts from the `casbin/casdoor-all-in-one` Docker image with the data in [.ci/casdoor/init_data.json](.ci/casdoor/init_data.json):

```shell
docker run -d --name casdoor -p 8000:8000 \
  -e driverName=sqlite \
  -e dataSourceName='file:casdoor.db?cache=shared' \
  -e initDataFile=/init_data.json \
  -v "$PWD/.ci/casdoor/init_data.json:/init_data.json:ro" \
  casbin/casdoor-all-in-one
```

## 📖 Documentation

- [Casdoor Official Documentation](https://casdoor.ai/docs/overview/)
- [Casdoor GitHub Repository](https://github.com/casdoor/casdoor)
- [API Documentation](https://door.casdoor.com/swagger)
- [docs.rs Reference](https://docs.rs/casdoor-rust-sdk)

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🌟 Star History

If you find this project useful, please consider giving it a star! ⭐

[![Star History Chart](https://api.star-history.com/svg?repos=casdoor/casdoor-rust-sdk&type=Date)](https://star-history.com/#casdoor/casdoor-rust-sdk&Date)
