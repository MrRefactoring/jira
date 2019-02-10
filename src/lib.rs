extern crate reqwest;

mod auth;
mod utils;

pub use auth::{ Credentials, AuthType };
pub use utils::{ ApiType };
pub use reqwest::{ Client, Method, StatusCode };

use reqwest::header::CONTENT_TYPE;

#[derive(Clone, Debug)]
pub struct RequestOptions {
    method: Method,
    api_type: ApiType,
    url: String
}

#[derive(Clone, Debug)]
pub struct Jira {
    host: String,
    credentials: Credentials,
    client: Client
}

impl Jira {
    pub fn new<H>(host: H, credentials: Credentials) -> Jira where H: Into<String> {
        Jira {
            host: host.into(),
            credentials,
            client: Client::new()
        }
    }

    fn send_request(&self, options: RequestOptions) {
        let request = self.client.request(options.method, &options.url);

        // TODO OAuth2
        let builder = match self.credentials {
            AuthType::Basic(ref username, ref password) => request
                .basic_auth(username.to_owned(), Some(password.to_owned()))
                .header(CONTENT_TYPE, "application/json")
        };

        let mut response = match body {
            Some(bod) => builder.body(bod).send()?,
            _ => builder.send()?
        };

        response.read_to_string(&mut body)?;

        match response.status() {
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::METHOD_NOT_ALLOWED => Err(Error::MethodNotAllowed),
            StatusCode::NOT_FOUND => Err(Error::NotFound),
        };
    }
}