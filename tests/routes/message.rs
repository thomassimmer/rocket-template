#[cfg(test)]
mod test {
    use crate::utils::TestContext;
    use rocket::http::{ContentType, Header, Status};
    use rocket_template::{self, json_string};
    use serde_json::Value;

    #[rocket::async_test]
    async fn test_all_methods() {
        let context = TestContext::new().await;

        // 0. User registers
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

        let body_str = post_response.into_string().await.unwrap();
        let body_json: Value = serde_json::from_str(&body_str).unwrap();
        let token = body_json["body"]["AuthToken"].as_str().unwrap();

        // 2. Create a message
        let post_response = context
            .client
            .post("/messages")
            .header(ContentType::JSON)
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .body(json_string!({
                "content": "Hello",
            }))
            .dispatch()
            .await;

        assert_eq!(post_response.status(), Status::Ok);

        let post_response_body = post_response.into_string().await.unwrap();

        assert!(post_response_body.contains("\"id\":1"));
        assert!(post_response_body.contains("\"author_id\":1"));
        assert!(post_response_body.contains("\"content\":\"Hello\""));

        // 2. Get all messages
        let get_all_response = context
            .client
            .get("/messages")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .dispatch()
            .await;

        assert_eq!(get_all_response.status(), Status::Ok);

        let get_all_response_body = get_all_response.into_string().await.unwrap();

        assert!(get_all_response_body.contains("\"id\":1"));
        assert!(get_all_response_body.contains("\"author_id\":1"));
        assert!(get_all_response_body.contains("\"content\":\"Hello\""));

        // 3. Get created message
        let get_response = context
            .client
            .get("/messages/1")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .dispatch()
            .await;

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
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .body(json_string!({
                "author_id": 1,
                "content": "Bonjour",
            }))
            .dispatch()
            .await;

        assert_eq!(put_response.status(), Status::Ok);

        let put_response_body = put_response.into_string().await.unwrap();

        assert!(put_response_body.contains("\"id\":1"));
        assert!(put_response_body.contains("\"author_id\":1"));
        assert!(put_response_body.contains("\"content\":\"Bonjour\""));

        // 5. Delete message
        let delete_response = context
            .client
            .delete("/messages/1")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .dispatch()
            .await;

        assert_eq!(delete_response.status(), Status::Ok);

        let delete_response_body = delete_response.into_string().await.unwrap();

        assert!(delete_response_body.contains("Ok"));

        // 6. Get all messages
        let get_all_response = context
            .client
            .get("/messages")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .dispatch()
            .await;

        assert_eq!(get_all_response.status(), Status::Ok);

        let get_all_response_body = get_all_response.into_string().await.unwrap();

        assert!(!get_all_response_body.contains("\"id\":1"));
        assert!(!get_all_response_body.contains("\"author_id\":1"));
        assert!(!get_all_response_body.contains("\"content\":\"Bonjour\""));
    }
}
