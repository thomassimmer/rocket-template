#[cfg(test)]
mod test {
    use crate::utils::TestContext;
    use rocket::http::{ContentType, Status};
    use rocket_template::{self, json_string};

    #[rocket::async_test]
    async fn test_all_methods() {
        let context = TestContext::new().await;

        // 0. Create a user
        let _post_user_response = context
            .client
            .post("/users")
            .header(ContentType::JSON)
            .body(json_string!({
                "first_name": "Thomas",
                "last_name": "Simmer",
            }))
            .dispatch()
            .await;

        // 2. Create a message
        let post_response = context
            .client
            .post("/messages")
            .header(ContentType::JSON)
            .body(json_string!({
                "author_id": 1, // TODO: This should be set automatically by looking at the request's author.
                "content": "Hello",
            }))
            .dispatch()
            .await;

        let post_response_body = post_response.into_string().await.unwrap();

        assert!(post_response_body.contains("\"id\":1"));
        assert!(post_response_body.contains("\"author_id\":1"));
        assert!(post_response_body.contains("\"content\":\"Hello\""));

        // 2. Get all messages
        let get_all_response = context.client.get("/messages").dispatch().await;

        assert_eq!(get_all_response.status(), Status::Ok);

        let get_all_response_body = get_all_response.into_string().await.unwrap();

        assert!(get_all_response_body.contains("\"id\":1"));
        assert!(get_all_response_body.contains("\"author_id\":1"));
        assert!(get_all_response_body.contains("\"content\":\"Hello\""));

        // 3. Get created message
        let get_response = context.client.get("/messages/1").dispatch().await;

        assert_eq!(get_response.status(), Status::Ok);

        let get_response_body = get_response.into_string().await.unwrap();

        assert!(get_response_body.contains("\"id\":1"));
        assert!(get_response_body.contains("\"author_id\":1"));
        assert!(get_response_body.contains("\"content\":\"Hello\""));

        // 4. Update message
        let put_response = context
            .client
            .put("/messages/1")
            .header(ContentType::JSON)
            .body(json_string!({
                "author_id": 1,
                "content": "Bonjour",
            }))
            .dispatch()
            .await;

        let put_response_body = put_response.into_string().await.unwrap();

        assert!(put_response_body.contains("\"id\":1"));
        assert!(put_response_body.contains("\"author_id\":1"));
        assert!(put_response_body.contains("\"content\":\"Bonjour\""));

        // 5. Delete message
        let delete_response = context.client.delete("/messages/1").dispatch().await;

        assert_eq!(delete_response.status(), Status::Ok);

        let delete_response_body = delete_response.into_string().await.unwrap();

        assert!(delete_response_body.contains("Ok"));

        // 6. Get all messages
        let get_all_response = context.client.get("/messages").dispatch().await;

        assert_eq!(get_all_response.status(), Status::Ok);

        let get_all_response_body = get_all_response.into_string().await.unwrap();

        assert!(!get_all_response_body.contains("\"id\":1"));
        assert!(!get_all_response_body.contains("\"author_id\":1"));
        assert!(!get_all_response_body.contains("\"content\":\"Bonjour\""));
    }
}
