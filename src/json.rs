use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse<C> {
    pub ms: u32,
    pub query: String,
    pub result: Vec<C>,
}
