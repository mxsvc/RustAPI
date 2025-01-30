pub mod funcs {
    use actix_web::{web, HttpResponse};
    use kuchiki::{iter::NodeIterator, parse_html, traits::TendrilSink};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    /// Expected Transform Request structure
    pub struct TransformRequest {
        pub transform: String,
        pub html: String,
    }

    /// Transform request handler
    #[actix_web::post("/transform")]
    pub async fn transform_handler(req: web::Json<TransformRequest>) -> HttpResponse {
        match transform_html(&req.transform, &req.html) {
            Ok(transformed_html) => HttpResponse::Ok()
                .content_type("text/plain")
                .body(transformed_html),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    /// Transform html as requested
    pub fn transform_html(transform: &str, html: &str) -> Result<String, &'static str> {
        let document = parse_html().one(html);

        // Apply transformations
        for node in document.select("p").map_err(|_| "Failed to parse HTML")? {
            for text_node in node.as_node().descendants().text_nodes() {
                let new_text = match transform {
                    "uppercase" => text_node.borrow().to_uppercase(),
                    "lowercase" => text_node.borrow().to_lowercase(),
                    _ => return Err("Invalid transform type"),
                };

                text_node.borrow_mut().clear();
                text_node.borrow_mut().push_str(&new_text);
            }
        }

        let mut result = document.to_string();

        // Remove unwanted wrapping tags
        result = result
            .replace("<html>", "")
            .replace("</html>", "")
            .replace("<head></head>", "")
            .replace("<body>", "")
            .replace("</body>", "")
            .trim()
            .to_string();

        Ok(result)
    }
}
