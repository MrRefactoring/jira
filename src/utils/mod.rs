#[derive(Clone, Debug)]
pub enum ApiType {
    Agile(String),
    Api(String),
    Auth(String),
    Webhook(String),
    DevInfo(String),
    FeatureFlag(String),
    Deployment(String),
    Builds(String)
}