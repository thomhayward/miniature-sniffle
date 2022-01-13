#[derive(Clone)]
pub(crate) struct Client {
    client: reqwest::Client,
    base: String,
    project: String,
    dataset: String,
    api: String,
    cdn: bool,
    token: Option<String>,
}

impl Client {
    pub(crate) fn new(client: reqwest::Client, project: &str, dataset: &str, api: &str) -> Client {
        Client {
            client,
            project: String::from(project),
            dataset: String::from(dataset),
            api: String::from(api),
            cdn: Default::default(),
            token: Default::default(),
            base: build_base_url(project, api, false),
        }
    }
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

    pub(crate) fn set_cdn(&mut self, use_cdn: bool) -> &mut Client {
        self.cdn = use_cdn;
        self.base = build_base_url(&self.project, &self.api, use_cdn);
        self
    }

    pub(crate) fn set_token(&mut self, token: &str) -> &mut Client {
        self.token = Some(String::from(token));
        self
    }
}

fn build_base_url(project: &str, api: &str, cdn: bool) -> String {
    format!(
        "https://{}.{}.sanity.io/v{}/data",
        project,
        if cdn { "apicdn" } else { "api" },
        api
    )
}
