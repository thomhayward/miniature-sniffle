
pub (crate) struct SanityClientInner {
    pub(crate) client: reqwest::Client,
    pub(crate) base: String,
    pub(crate) project: String,
    pub(crate) dataset: String,
    pub(crate) api: String,
    pub(crate) cdn: bool,
    pub(crate) token: Option<String>,
}

impl SanityClientInner {
    pub(crate) fn begin(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        path: &str,
    ) -> reqwest::RequestBuilder {
        let url = format!("{}/{}/{}/{}", &self.base, endpoint, &self.dataset, path);
        if let Some(token) = &self.token {
            return self.client.request(method, url).bearer_auth(token);
        }
        self.client.request(method, url)
    }
}
