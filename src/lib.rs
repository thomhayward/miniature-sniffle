use reqwest::Client;
use std::sync::Arc;

mod query;
use query::QueryBuilder;

mod documents;
use documents::DocumentsBuilder;

#[derive(Debug)]
struct SanityClientInner {
    client: Client,
    base: String,
    project: String,
    dataset: String,
    api: String,
    cdn: bool,
    token: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SanityClient {
    inner: Arc<SanityClientInner>,
}

fn build_base_url(project: &str, api: &str, cdn: bool) -> String {
    format!(
        "https://{}.{}.sanity.io/v{}/data",
        project,
        if cdn { "apicdn" } else { "api" },
        api
    )
}

impl SanityClient {
    /// Create a new client with the specified project_id, dataset, and api version.
    ///
    pub fn new(project: &str, dataset: &str, api: &str) -> SanityClient {
        SanityClient {
            inner: Arc::new(SanityClientInner {
                client: Client::new(),
                base: build_base_url(project, api, false),
                project: String::from(project),
                dataset: String::from(dataset),
                api: String::from(api),
                cdn: false,
                token: None,
            }),
        }
    }
    /// Set the authentication token
    ///
    /// [https://www.sanity.io/docs/http-auth]
    #[must_use]
    pub fn token(self, token: &str) -> SanityClient {
        let SanityClientInner {
            project,
            dataset,
            api,
            cdn,
            ..
        } = &*self.inner;
        SanityClient {
            inner: Arc::new(SanityClientInner {
                client: Client::new(),
                base: build_base_url(project, api, *cdn),
                project: String::from(project),
                dataset: String::from(dataset),
                api: String::from(api),
                cdn: false,
                token: Some(String::from(token)),
            }),
        }
    }
    /// Set or unset use of the API CDN.
    ///
    /// [https://www.sanity.io/docs/api-cdn]
    #[must_use]
    pub fn cdn(self, use_cdn: bool) -> SanityClient {
        let SanityClientInner {
            project,
            dataset,
            api,
            token,
            ..
        } = &*self.inner;
        SanityClient {
            inner: Arc::new(SanityClientInner {
                client: Client::new(),
                base: build_base_url(project, api, use_cdn),
                project: String::from(project),
                dataset: String::from(dataset),
                api: String::from(api),
                cdn: use_cdn,
                token: token.as_ref().map(|value| String::from(value)),
            }),
        }
    }
    /// Create a QueryBuilder to execute queries on the 'Query' endpoint.
    ///
    /// [https://www.sanity.io/docs/http-query]
    pub fn query<'a>(&self, query: &'a str) -> QueryBuilder<'a> {
        QueryBuilder::new(self.inner.clone(), query)
    }
    /// Create a DocumentsBuilder to query documents by id from the 'Doc' endpoint.
    ///
    /// [https://www.sanity.io/docs/http-doc]
    pub fn documents<'a>(&self, ids: &[&'a str]) -> DocumentsBuilder<'a> {
        DocumentsBuilder::new(self.inner.clone(), ids)
    }
    pub fn document<'a>(&self, id: &'a str) -> DocumentsBuilder<'a> {
        DocumentsBuilder::new(self.inner.clone(), &[id])
    }
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
