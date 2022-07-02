use casdoor::{networkconfig, user, CasdoorSDK};

#[test]
fn test_get_auth_link() {
    let app = CasdoorSDK::new(
        "http://127.0.0.1:8000",
        "5dfdb2b205c8d7efdf26",
        "52494374aad2e2e6942e4cc307347d5b257b1598",
        Box::new([0]),
        "testsdk",
        Some("http://127.0.0.1:7001"),
        None,
        None,
    );
    let config = networkconfig!("http://localhost:9000/callback", "testsdk_a1");
    let url = app.get_auth_link(&config).unwrap();
    assert_eq!(
        url,
        "http://127.0.0.1:7001/login/oauth/authorize?client_id=5dfdb2b205c8d7efdf26&redirect_uri=http://localhost:9000/callback&response_type=code&scope=read&state=testsdk_a1"
    )
}

//something wrong with this
#[tokio::test]
async fn test_oauth_token() {
    let certificate = br#""#;
    let code = "";
    let app = CasdoorSDK::new(
        "http://127.0.0.1:8000",
        "5dfdb2b205c8d7efdf26",
        "52494374aad2e2e6942e4cc307347d5b257b1598",
        Box::new(certificate.to_owned()),
        "testsdk",
        Some("http://127.0.0.1:7001"),
        None,
        None,
    );
    let access_key = app.get_oauth_token(code).await.unwrap();
    println!("{}", access_key);
    let claim = match app.parse_jwt_token(&access_key) {
        Ok(x) => x,
        Err(err) => {
            println!("{:?}", err);
            panic!();
        }
    };
    println!("{:?}", claim);
}

#[tokio::test]
async fn test_jwt() {
    let access_key = r#""#;
    let certificate = br#""#;
    let app = CasdoorSDK::new(
        "http://127.0.0.1:8000",
        "5dfdb2b205c8d7efdf26",
        "52494374aad2e2e6942e4cc307347d5b257b1598",
        Box::new(certificate.to_owned()),
        "testsdk",
        Some("http://127.0.0.1:7001"),
        None,
        None,
    );
    let user_ans = user!(
        owner = "testsdk".to_owned(),
        name = "sunyh".to_owned(),
        created_time = "2022-06-26T21:08:18+08:00".to_owned(),
        id = "8a580f76-c8a8-4248-9651-a068a32d2b77".to_owned(),
        r#type = "normal-user".to_owned(),
        display_name = "New User - qew3fb".to_owned(),
        avatar = "https://casbin.org/img/casbin.svg".to_owned(),
        email = "leavelet0@gmail.com".to_owned(),
        email_verified = false,
        phone = "24107502942".to_owned(),
        affiliation = "Example Inc.".to_owned(),
        region = "CN".to_owned(),
        score = 0,
        karma = 0,
        ranking = 2,
        is_default_avatar = false,
        is_online = false,
        is_admin = true,
        is_global_admin = false,
        is_forbidden = false,
        is_deleted = false,
        signup_application = "app-built-in".to_owned(),
        tag = "staff".to_owned()
    );
    let claim = match app.parse_jwt_token(&access_key) {
        Ok(x) => x,
        Err(err) => {
            println!("{:?}", err);
            panic!();
        }
    };
    assert_eq!(claim, user_ans);
}

#[tokio::test]
async fn test_get_users() {
    let certificate = br#""#;
    let app = CasdoorSDK::new(
        "http://127.0.0.1:8000",
        "5dfdb2b205c8d7efdf26",
        "52494374aad2e2e6942e4cc307347d5b257b1598",
        Box::new(certificate.to_owned()),
        "testsdk",
        Some("http://127.0.0.1:7001"),
        None,
        None,
    );
    let x = app.get_users().await.unwrap();
    let user_ans = user!(
        owner = "testsdk".to_owned(),
        name = "sunyh".to_owned(),
        created_time = "2022-06-26T21:08:18+08:00".to_owned(),
        id = "8a580f76-c8a8-4248-9651-a068a32d2b77".to_owned(),
        r#type = "normal-user".to_owned(),
        display_name = "New User - qew3fb".to_owned(),
        avatar = "https://casbin.org/img/casbin.svg".to_owned(),
        email = "leavelet0@gmail.com".to_owned(),
        email_verified = false,
        phone = "24107502942".to_owned(),
        affiliation = "Example Inc.".to_owned(),
        region = "CN".to_owned(),
        score = 0,
        karma = 0,
        ranking = 2,
        is_default_avatar = false,
        is_online = false,
        is_admin = true,
        is_global_admin = false,
        is_forbidden = false,
        is_deleted = false,
        password = "***".to_owned(),
        signup_application = "app-built-in".to_owned(),
        tag = "staff".to_owned()
    );
    assert_eq!(x[0], user_ans);
}

#[tokio::test]
async fn test_get_user_by_name() {
    let certificate = br#""#;
    let app = CasdoorSDK::new(
        "http://127.0.0.1:8000",
        "5dfdb2b205c8d7efdf26",
        "52494374aad2e2e6942e4cc307347d5b257b1598",
        Box::new(certificate.to_owned()),
        "testsdk",
        Some("http://127.0.0.1:7001"),
        None,
        None,
    );
    let user = app.get_user_by_name("sunyh").await.unwrap();
    let user_ans = user!(
        owner = "testsdk".to_owned(),
        name = "sunyh".to_owned(),
        created_time = "2022-06-26T21:08:18+08:00".to_owned(),
        id = "8a580f76-c8a8-4248-9651-a068a32d2b77".to_owned(),
        r#type = "normal-user".to_owned(),
        display_name = "New User - qew3fb".to_owned(),
        avatar = "https://casbin.org/img/casbin.svg".to_owned(),
        email = "leavelet0@gmail.com".to_owned(),
        email_verified = false,
        phone = "24107502942".to_owned(),
        affiliation = "Example Inc.".to_owned(),
        region = "CN".to_owned(),
        score = 0,
        karma = 0,
        ranking = 2,
        is_default_avatar = false,
        is_online = false,
        is_admin = true,
        is_global_admin = false,
        is_forbidden = false,
        is_deleted = false,
        password = "***".to_owned(),
        signup_application = "app-built-in".to_owned(),
        tag = "staff".to_owned()
    );
    match user {
        Some(user) => {
            assert_eq!(user, user_ans);
        }
        _ => {
            panic!("no such user!");
        }
    }
}

#[tokio::test]
async fn test_modify_user() {
    let certificate = br#""#;
    let app = CasdoorSDK::new(
        "http://127.0.0.1:8000",
        "5dfdb2b205c8d7efdf26",
        "52494374aad2e2e6942e4cc307347d5b257b1598",
        Box::new(certificate.to_owned()),
        "testsdk",
        Some("http://127.0.0.1:7001"),
        None,
        None,
    );
    let user = app.get_user_by_name("tmp").await.unwrap();
    match user {
        Some(user) => {
            println!("{:?}", user);
        }
        None => {
            println!("no such user!");
        }
    }
    let mut user_now = user!(name = "tmp".to_owned());
    let res = app.add_user(&mut user_now).await.unwrap();
    assert_eq!(http::StatusCode::OK, res.status());
    let user_get = app.get_user_by_name("tmp").await.unwrap().unwrap();
    assert_eq!(user_now.name, user_get.name);
    let res = app.delete_user(&mut user_now).await.unwrap();
    assert_eq!(http::StatusCode::OK, res.status());
    let user_now = app.get_user_by_name("tmp").await.unwrap();
    assert_eq!(user_now, None);
}
