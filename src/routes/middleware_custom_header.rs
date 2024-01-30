use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};

pub async fn middleware_custom_header(headers: HeaderMap, request: Request,next: Next) -> Result<Response, String>{
    let token_option= headers.get("token");

    match token_option {
        Some(val) => {
            dbg!(" Value of token is: {}",val);
            let response = next.run(request).await;
            Ok(response)
        }
        _ => {Err("Token invalid".to_owned())}
    }
}