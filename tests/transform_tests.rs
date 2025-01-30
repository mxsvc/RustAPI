#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};
    use rust_test::funcs::transform_handler;

    /// Send a transform request for testing
    async fn send_transform_request(request_body: &str) -> (u16, String) {
        // TODO: Reuse same test app
        let app = test::init_service(App::new().service(transform_handler)).await;

        let req = test::TestRequest::post()
            .uri("/transform")
            .insert_header(ContentType::json())
            .set_payload(request_body.to_string().into_bytes()) // Convert &str -> String -> Bytes
            .to_request();

        let response = test::call_service(&app, req).await;
        let status = response.status().as_u16();
        let body = test::read_body(response).await;

        (
            status,
            String::from_utf8(body.to_vec()).expect("Invalid UTF-8 response"),
        )
    }

    #[actix_web::test]
    async fn test_uppercase_transform() {
        let request_body = r#"
        {
            "transform": "uppercase",
            "html": "<p>Hello world</p>"
        }
        "#;

        let expected_body = "<p>HELLO WORLD</p>";

        let (status, body) = send_transform_request(request_body).await;

        assert_eq!(status, 200);
        assert_eq!(body, expected_body);
    }

    #[actix_web::test]
    async fn test_lowercase_transform() {
        let request_body = r#"
        {
            "transform": "lowercase",
            "html": "<p>Hello WORLD</p>"
        }
        "#;

        let expected_body = "<p>hello world</p>";

        let (status, body) = send_transform_request(request_body).await;

        assert_eq!(status, 200);
        assert_eq!(body, expected_body);
    }

    #[actix_web::test]
    async fn test_multiple_paragraphs() {
        let request_body = r#"
        {
            "transform": "uppercase",
            "html": "<div><p>First paragraph</p><span>Not a paragraph</span><p>Second paragraph</p></div>"
        }
        "#;

        let expected_body =
            "<div><p>FIRST PARAGRAPH</p><span>Not a paragraph</span><p>SECOND PARAGRAPH</p></div>";

        let (status, body) = send_transform_request(request_body).await;

        assert_eq!(status, 200);
        assert_eq!(body, expected_body);
    }

    #[actix_web::test]
    async fn test_nested_elements() {
        let request_body = r#"
        {
            "transform": "uppercase",
            "html": "<p>Text with <strong>bold</strong> and <em>italic</em> elements</p>"
        }
        "#;

        let expected_body = "<p>TEXT WITH <strong>BOLD</strong> AND <em>ITALIC</em> ELEMENTS</p>";

        let (status, body) = send_transform_request(request_body).await;

        assert_eq!(status, 200);
        assert_eq!(body, expected_body);
    }
}
