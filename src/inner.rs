use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Method, RequestBuilder};

#[derive(Clone)]
pub(crate) struct Client {
    client: reqwest::Client,
    base: String,
    project: String,
    dataset: String,
    api: String,
    cdn: bool,
}

impl Client {
    pub(crate) fn new(client: reqwest::Client, project: &str, dataset: &str, api: &str) -> Client {
        Client {
            client,
            project: String::from(project),
            dataset: String::from(dataset),
            api: String::from(api),
            cdn: Default::default(),
            base: build_base_url(project, api, false),
        }
    }

    pub(crate) fn begin(&self, method: Method, endpoint: &str, path: &str) -> RequestBuilder {
        let url = format!("{}/{}/{}/{}", &self.base, endpoint, &self.dataset, path);
        self.client.request(method, url)
    }

    pub(crate) fn set_cdn(&mut self, use_cdn: bool) -> &mut Client {
        self.cdn = use_cdn;
        self.base = build_base_url(&self.project, &self.api, use_cdn);
        self
    }

    pub(crate) fn set_token(&mut self, token: &str) -> &mut Client {
        // Construct an 'Authorization: Bearer <token>' header
        let mut bearer = HeaderValue::from_str(&format!("Bearer {}", token))
            .expect("inner::Client::set_token(), error building HeaderValue from token");
        bearer.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, bearer);
        // Build a new client with the authorization header as a default header
        let new_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("inner::Client::set_token(), error building reqwest::Client");
        self.client = new_client;
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
