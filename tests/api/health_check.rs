use crate::helpers::{clean_up_database, spawn_app};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Acct
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    clean_up_database(&app.db_settings).await;
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
