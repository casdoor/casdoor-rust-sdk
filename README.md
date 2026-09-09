# casdoor-rust-sdk

[![GitHub last commit](https://img.shields.io/github/last-commit/casdoor/casdoor-rust-sdk)](https://github.com/casdoor/casdoor-rust-sdk/commits/master)
[![Crates.io](https://img.shields.io/crates/v/casdoor-rust-sdk.svg)](https://crates.io/crates/casdoor-rust-sdk)
[![Docs](https://docs.rs/casdoor-rust-sdk/badge.svg)](https://docs.rs/casdoor-rust-sdk)
[![CI](https://github.com/casdoor/casdoor-rust-sdk/workflows/CI/badge.svg)](https://github.com/casdoor/casdoor-rust-sdk/actions)
[![Discord](https://img.shields.io/discord/1022748306096537660?logo=discord&label=discord&color=5865F2)](https://discord.gg/5rPsrAzK7S)

This is Casdoor's SDK for Rust, which will allow you to easily connect your application to the Casdoor authentication system without having to implement it from scratch.

The API follows the [Casdoor Go SDK](https://github.com/casdoor/casdoor-go-sdk): the same objects, the same actions, one Rust module per Go file, so the two SDKs stay easy to keep in sync.

```toml
[dependencies]
casdoor-rust-sdk = "2"
```

## Step1. Init the client

Initialization requires 6 parameters, which are all of the string type:

| Name (in order)   | Must | Description                                         |
| ----------------- | ---- | --------------------------------------------------- |
| endpoint          | Yes  | Casdoor server URL, such as `http://localhost:8000` |
| client_id         | Yes  | Client ID of the Casdoor application                |
| client_secret     | Yes  | Client secret of the Casdoor application            |
| certificate       | Yes  | x509 certificate content of the application's cert  |
| organization_name | Yes  | The name of the Casdoor organization                |
| application_name  | No   | The name of the Casdoor application                 |

```rust
use casdoor_rust_sdk::Client;

// Init from the parameters.
let client = Client::new(endpoint, client_id, client_secret, certificate, organization_name, application_name);

// Or init from a TOML file, the path should be an absolute path. (recommended)
let client = Client::from_toml(path)?;
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

## Step2. Sign the user in

```rust
// Redirect the user to this URL to sign in, Casdoor sends them back to the redirect URI
// with a `code` query parameter.
let signin_url = client.get_signin_url("http://localhost:9000/callback");

// In the callback, exchange the code for an access token.
let token = client.get_oauth_token(&code).await?;

// Verify the token with the application's certificate and read the user out of it.
let claims = client.parse_jwt_token(&token.access_token)?;
println!("signed in as {}", claims.user.name);

// Sign the user out of all their sessions (single sign-out).
client.logout(&token.access_token).await?;
```

The other ways to get a token are `refresh_oauth_token()`, `get_oauth_token_by_password()` (the `password` grant type) and `impersonate_user()` (an admin acting as a user).

## Step3. Call the APIs

By default the client calls the APIs as the application itself, using the client ID and the client secret. `with_access_token()` returns a copy of the client that calls them as the user who owns the access token instead:

```rust
let user = client.with_access_token(&token.access_token).get_account().await?;
```

All the API methods are `async` and return `Result<T, CasdoorError>`. The getters of a single object return `Option<T>`, which is `None` when the object doesn't exist:

```rust
// Users
let users = client.get_users().await?;
let (users, count) = client.get_pagination_users(1, 10, &[]).await?;
let user = client.get_user("admin").await?;
let user = client.get_user_by_email("admin@example.com").await?;
client.add_user(&user).await?;
client.update_user(&user).await?;
client.delete_user(&user).await?;
client.set_password("built-in", "admin", "old-password", "new-password").await?;

// Permissions
let allowed = client.enforce("built-in/permission", "", "", "", "", &request).await?;

// Messages
client.send_email("title", "content", "sender", &["alice@example.com"]).await?;
client.send_sms("content", &["12345678910"]).await?;
client.send_notification("content", "").await?;

// Files
let (url, name) = client.upload_resource("admin", "tag", "", "/avatar.png", &bytes).await?;
```

The same `get_xxx()` / `get_xxxs()` / `get_pagination_xxxs()` / `add_xxx()` / `update_xxx()` / `delete_xxx()` methods exist for every Casdoor object:

adapter, application, cert, enforcer, group, invitation, LDAP server, model, order, organization, payment, permission, plan, policy, pricing, product, provider, record, resource, role, session, subscription, syncer, token, transaction, user and webhook.

There are also the MFA APIs (`initiate_mfa()`, `verify_mfa()`, `enable_mfa()`, `set_preferred_mfa()`, `delete_mfa()`), the payment APIs (`place_order()`, `pay_order()`) and the token introspection API (`introspect_token()`).

## Migrating from 1.x

The 2.0 version replaces `AuthService` and `UserService` with a single `Client`, which covers all the Casdoor objects instead of only the users, and makes every API return a typed `CasdoorError` instead of a `Box<dyn Error>`.

| 1.x                                     | 2.0                                          |
| --------------------------------------- | -------------------------------------------- |
| `CasdoorConfig::new(...)`               | `Client::new(...)`                           |
| `CasdoorConfig::from_toml(path)`        | `Client::from_toml(path)`                    |
| `CasdoorUser`                           | `User`                                       |
| `AuthService::new(&conf)`               | the `Client` itself                          |
| `auth.get_auth_token(code)`             | `client.get_oauth_token(&code).await?`       |
| `auth.parse_jwt_token(token)`           | `client.parse_jwt_token(&token)?`            |
| `auth.get_signin_url(uri)`              | `client.get_signin_url(&uri)`                |
| `auth.get_signup_url_enable_password()` | `client.get_signup_url(true, "")`            |
| `auth.get_user_profile_url(name, tok)`  | `client.get_user_profile_url(&name, tok)`    |
| `UserService::new(&conf)`               | the `Client` itself                          |
| `user_service.get_user(name)`           | `client.get_user(&name).await?` (an `Option`) |
| `user_service.get_user_with_email(...)` | `client.get_user_by_email(&email).await?`    |
| `user_service.add_user(user)`           | `client.add_user(&user).await?` (a `bool`)   |

The certificate is now parsed as a real x509 certificate, so the certificate copied from Casdoor's "Certs" page works as-is, and a bare `PUBLIC KEY` PEM is still accepted.

## Development

```shell
cargo test
```

The tests that need a running Casdoor server are marked as ignored, run them with `cargo test -- --ignored`. They read the server config from the `CASDOOR_TEST_ENDPOINT`, `CASDOOR_TEST_CLIENT_ID`, `CASDOOR_TEST_CLIENT_SECRET`, `CASDOOR_TEST_ORGANIZATION` and `CASDOOR_TEST_APPLICATION` environment variables.
