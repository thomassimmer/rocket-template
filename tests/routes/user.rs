#[cfg(test)]
mod test {
    use crate::utils::TestContext;
    use rocket::http::{ContentType, Status};
    use rocket_template::{self, json_string};

    #[rocket::async_test]
    async fn test_all_methods() {
        let context = TestContext::new().await;

        // 1. Create a user
        let post_response = context
            .client
            .post("/users")
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

        let post_response_body = post_response.into_string().await.unwrap();

        assert!(post_response_body.contains("\"id\":1"));
        assert!(post_response_body.contains("\"first_name\":\"Thomas\""));
        assert!(post_response_body.contains("\"last_name\":\"Simmer\""));

        // 2. Get all users
        let get_all_response = context.client.get("/users").dispatch().await;

        assert_eq!(get_all_response.status(), Status::Ok);

        let get_all_response_body = get_all_response.into_string().await.unwrap();

        assert!(get_all_response_body.contains("\"id\":1"));
        assert!(get_all_response_body.contains("\"first_name\":\"Thomas\""));
        assert!(get_all_response_body.contains("\"last_name\":\"Simmer\""));

        // 3. Get created user
        let get_response = context.client.get("/users/1").dispatch().await;

        assert_eq!(get_response.status(), Status::Ok);

        let get_response_body = get_response.into_string().await.unwrap();

        assert!(get_response_body.contains("\"id\":1"));
        assert!(get_response_body.contains("\"first_name\":\"Thomas\""));
        assert!(get_response_body.contains("\"last_name\":\"Simmer\""));

        // 4. Update user
        let put_response = context
            .client
            .put("/users/1")
            .header(ContentType::JSON)
            .body(json_string!({
                "last_name": "Merci",
            }))
            .dispatch()
            .await;

        assert_eq!(put_response.status(), Status::Ok);

        let put_response_body = put_response.into_string().await.unwrap();

        assert!(put_response_body.contains("\"id\":1"));
        assert!(put_response_body.contains("\"first_name\":\"Thomas\""));
        assert!(put_response_body.contains("\"last_name\":\"Merci\""));

        // 5. Delete user
        let delete_response = context.client.delete("/users/1").dispatch().await;

        assert_eq!(delete_response.status(), Status::Ok);

        let delete_response_body = delete_response.into_string().await.unwrap();

        assert!(delete_response_body.contains("Ok"));

        // 6. Get all users
        let get_all_response = context.client.get("/users").dispatch().await;

        assert_eq!(get_all_response.status(), Status::Ok);

        let get_all_response_body = get_all_response.into_string().await.unwrap();

        assert!(!get_all_response_body.contains("\"id\":1"));
        assert!(!get_all_response_body.contains("\"first_name\":\"Thomas\""));
        assert!(!get_all_response_body.contains("\"last_name\":\"Merci\""));
    }
}
