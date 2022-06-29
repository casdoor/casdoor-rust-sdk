# casdoor-rust-sdk

This is Casdoor's SDK for Rust, which will allow you to easily connect your application to the Casdoor authentication system without having to implement it from scratch.

Casdoor SDK is very simple to use. We will show you the steps below.

```
[dependencies]
casdoor = "0.1"

//in your app
[macro_use]
extern crate casdoor;
```

## Step1. Init SDK

Initialization requires 6 parameters, which are all string type:

| Name (in order)  | Must | Description                                                        |
| ---------------- | ---- | ------------------------------------------------------------------ |
| endpoint         | Yes  | Casdoor Server Url, such as `http://localhost:8000`                |
| client_id        | Yes  | Application.client_id                                              |
| client_secret    | Yes  | Application.client_secret                                          |
| certificate      | Yes  | Same as your Application Certificate                               |
| org_name         | Yes  | Application.organization                                           |
| front_endpoint   | No   | Casdoor web endpoint, default is endpoint.replace(":8000", ":7001")|

```rust
//anything implemented "ToString" trait can be used by this macro
let app = newsdk!(endpoint, client_id, client_secret, certificate, org_name);
```

## Step2. Get token and parse

After casdoor verification passed, it will be redirected to your application with code and state, like `http://forum.casbin.org?code=xxx&state=yyyy`.

Your web application can get the `code`,`state` and call `GetOAuthToken(code, state)`, then parse out jwt token.
Hints:  

1. `redirect_uri` is the URL that your `APP` is configured to
listen to the response from `Casdoor`. For example, if your `redirect_uri` is `https://forum.casbin.com/callback`, then Casdoor will send a request to this URL along with two parameters `code` and `state`, which will be used in later steps for authentication.   

2. `state` is usually your Application's name, you can find it under the `Applications` tab in `Casdoor`, and the leftmost `Name` column gives each application's name. 

3. Of course you want your `APP` to be able to send the URL. For example you should have something like a button, and it carries this URL. So when you click the button, you should be redirected to `Casdoor` for verification. For now you are typing it in the browser simply for testing.
   
4. all the function will return `Result<_, Box<dyn std::error::Error>>`, as there are mutiple kinds of error possible.

The general process is as follows:

```rust
//config will generate a NetWorkConfig struct, default response type is "code", and default scope is "read"
let config = &networkconfig!(callback, state)
let link = app.get_auth_link(&config)?;
//and the code will be sent to callback address
let code = (your method to get code);
let access_token = app.get_oauth_token(&code)?;
//the claims returns a HashMap<String, String>
let claims = parse_jwt_token(access_token)?;


```

## Step3. Interact with the users

The sdk support basic user operations, like:

- `get_user_by_name(name)`, get one user by user name.
- `get_users()`, get all users.
- `add_user(User)`, write user to database.
- `update_user(User)`, update user info
- `delete_user(User)`, delete user
