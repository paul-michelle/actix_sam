use actix_sam::startup::APIStage;
use fake::faker::{internet::en::SafeEmail, name::raw::Name};
use fake::locales::{EN, FR_FR, JA_JP};
use fake::Fake;
use once_cell::sync::Lazy;
use reqwest::header;
use std::collections::HashMap;

use actix_sam::{
    startup,
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING_INIT_ONCE: Lazy<()> = Lazy::new(|| {
    let test_log_directive = std::env::var("TEST_LOG");
    if test_log_directive.is_ok() {
        let subscriber =
            get_subscriber("TEST".into(), test_log_directive.unwrap(), std::io::stdout);
        init_subscriber(subscriber);
        return;
    };
    let subscriber = get_subscriber("TEST".into(), "debug".into(), std::io::sink);
    init_subscriber(subscriber);
});

const HEALTH_CHECK_ENDPOINT: &'static str = "/health_check";
const SUBSCRIBTIONS_ENDPOINT_V1: &'static str = "/subscriptions";

struct TestApp {
    address: String,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING_INIT_ONCE);

    let listener = std::net::TcpListener::bind("localhost:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();

    let awaitable_server = startup::run(listener)
        .await
        .expect("Failed to bind address");

    tokio::spawn(awaitable_server);

    TestApp {
        address: format!("http://localhost:{}/{}", port, APIStage::Dev.as_str()),
    }
}

#[tokio::test]
async fn health_check_is_ok() {
    let test_app = spawn_app().await;

    let resp = reqwest::Client::new()
        .get(format!("{}{}", test_app.address, HEALTH_CHECK_ENDPOINT))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.content_length(), Some(0));
}

#[tokio::test]
async fn subsriptions_endpoint_errors_back_on_missing_or_invalid_details() {
    let email_faker = SafeEmail();

    let test_cases = vec![
        (
            format!("email={}", email_faker.fake::<String>()),
            "name is missing",
        ),
        (
            format!("email={}&name=", email_faker.fake::<String>()),
            "name is empty string",
        ),
        (
            format!("email={}&name= ", email_faker.fake::<String>()),
            "name is white space",
        ),
        (
            format!("email={}&name=<turbo>", email_faker.fake::<String>()),
            "name is contains angle brackets",
        ),
        (
            format!("name={}", Name(JA_JP).fake::<String>()),
            "email is missing",
        ),
        (
            format!(
                "name={}&email=rob.pikegmail.com",
                Name(FR_FR).fake::<String>()
            ),
            "@ sign missing in email address",
        ),
        (
            format!("name={}&email=@gmail.com", Name(EN).fake::<String>()),
            "subject missing in email address",
        ),
    ];
    let client = reqwest::Client::new();
    let test_app = spawn_app().await;

    for (subscriber_details, msg) in test_cases {
        let resp = client
            .post(format!("{}{}", test_app.address, SUBSCRIBTIONS_ENDPOINT_V1))
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(subscriber_details)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            resp.status().as_u16(),
            400,
            "Status is not 400 though {msg}"
        );
    }
}

#[tokio::test]
async fn subsriptions_endpoint_returns_ok_for_valid_inputs() {
    let subscriber_name = Name(JA_JP).fake::<String>();
    let subscriber_email = SafeEmail().fake::<String>();
    let mut form = HashMap::new();
    form.insert("name", format!("{}", subscriber_name));
    form.insert("email", format!("{}", subscriber_email));

    let client = reqwest::Client::new();
    let test_app = spawn_app().await;

    let resp = client
        .post(format!("{}{}", test_app.address, SUBSCRIBTIONS_ENDPOINT_V1))
        .form(&form)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(resp.status().as_u16(), 201);

    // identifier retuned
    let resp_data = resp.json::<HashMap<String, String>>().await.unwrap();
    let inserted_id = resp_data.get("id");
    assert!(inserted_id.is_some());
}
