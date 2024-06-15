#[cfg(test)]
mod test {
    use rocket::http::Status;

    use crate::utils::TestContext;

    #[rocket::async_test]
    async fn hello_world() {
        let context = TestContext::new().await;
        let response = context.client.get("/").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await, Some("Rocket-template is running.".into()));
    }

    #[rocket::async_test]
    async fn error404() {
        let context = TestContext::new().await;
        let response = context.client.get("/not_existing_path").dispatch().await;
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(
            response.into_string().await,
            Some("Sorry, '/not_existing_path' is not a valid path.".into())
        );
    }
}
