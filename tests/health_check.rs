
#[tokio::test]
async fn health_check_works() {
    //Arrange
    spawn_app().await.expect("Failed to spawn our app.");

    // we need to bring in `reqwest`
    // tp perform HTTP requests against our application
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success() );
    assert_eq!(Some(0), response.content_length());
}

//launch our application in the background
async fn spawn_app() -> Result<(), std::io::Error> {
    zero2prod::run().await
}