#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket_template::json_string;

    use crate::utils::test_context::TestContext;

    #[rocket::async_test]
    async fn hello_world() {
        let context = TestContext::new().await;
        let response = context.client.get("/").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await,
            Some("Rocket-template is running.".into())
        );
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

    #[rocket::async_test]
    async fn test_register() {
        let context = TestContext::new().await;

        // 1. User registers
        let post_response = context
            .client
            .post("/register")
            .header(ContentType::JSON)
            .body(json_string!({
                "user_name": "thomas.simmer",
                "password": "password",
                "first_name": "Thomas",
                "last_name": "Simmer",
            }))
            .dispatch()
            .await;

        assert_eq!(post_response.status(), Status::Ok);
        assert!(post_response
            .into_string()
            .await
            .unwrap()
            .contains("{\"body\":{\"AuthToken\":"));
    }

    #[rocket::async_test]
    async fn test_login() {
        let context = TestContext::new().await;

        // 1. User registers
        let post_response = context
            .client
            .post("/register")
            .header(ContentType::JSON)
            .body(json_string!({
                "user_name": "thomas.simmer",
                "password": "password",
                "first_name": "Thomas",
                "last_name": "Simmer",
            }))
            .dispatch()
            .await;
        assert_eq!(post_response.status(), Status::Ok);

        // 2. User logins with the good credentials
        let post_response = context
            .client
            .post("/login")
            .header(ContentType::JSON)
            .body(json_string!({
                "user_name": "thomas.simmer",
                "password": "password",
            }))
            .dispatch()
            .await;

        assert_eq!(post_response.status(), Status::Ok);
        assert!(post_response
            .into_string()
            .await
            .unwrap()
            .contains("{\"body\":{\"AuthToken\":"));

        // 3. User logins with wrong credentials
        let post_response = context
            .client
            .post("/login")
            .header(ContentType::JSON)
            .body(json_string!({
                "user_name": "thomas.simmer",
                "password": "wrong_password",
            }))
            .dispatch()
            .await;

        assert_eq!(post_response.status(), Status::NotFound);
        assert!(post_response.into_string().await.unwrap().contains(
            "{\"body\":{\"Message\":\"Error - Wrong username or password for user thomas.simmer\"}"
        ));
    }
}
