use casdoor::{networkconfig, CasdoorSDK};

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
