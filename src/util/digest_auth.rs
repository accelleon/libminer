// Helper function to generate a digest auth header

use async_trait::async_trait;
use reqwest::{RequestBuilder, Response, StatusCode};
use digest_auth::AuthContext;
use crate::error::Error;

#[async_trait]
pub trait WithDigestAuth {
    async fn send_with_digest_auth(self, username: &str, password: &str) -> Result<Response, Error>;
}

#[async_trait]
impl WithDigestAuth for RequestBuilder {
    //TODO: this can panic
    async fn send_with_digest_auth(self, username: &str, password: &str) -> Result<Response, Error> {
        // Send a request to get the digest auth headers
        let req = self.try_clone().unwrap();//.send().await?;
        let resp = req.send().await?;
        match resp.status() {
            StatusCode::UNAUTHORIZED => {
                let request = self.try_clone().unwrap().build()?;
                let uri = request.url().path();
                let method = digest_auth::HttpMethod::from(request.method().as_str());
                let body = request.body().and_then(|b| b.as_bytes());
                let www_auth = resp.headers().get("www-authenticate").unwrap().to_str()?;
                let context = AuthContext::new_with_method(username, password, uri, body, method);
                let mut prompt = digest_auth::parse(www_auth)?;
                let auth_header = prompt.respond(&context)?;
                Ok(self.header("Authorization", auth_header.to_header_string()).send().await?)
            }
            _ => return Ok(resp),
        }
    }
}
