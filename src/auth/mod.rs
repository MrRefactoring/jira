#[derive(Clone, Debug)]
pub enum AuthType {
    Basic,
    OAuth
}

#[derive(Clone, Debug)]
pub struct Basic {
    username: String,
    password: String
}

#[derive(Clone, Debug)]
pub struct OAuth {

}

#[derive(Clone, Debug)]
pub struct Credentials {
    consumer_key: String,
    private_key: String,
    token: String,
    token_secret: String
}

impl Credentials {
    pub fn new(auth_type: AuthType) {

    }
}
