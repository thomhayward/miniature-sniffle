use std::sync::Arc;

mod endpoints;
mod inner;
pub use endpoints::{DocumentsBuilder, QueryBuilder};

pub struct Client {
    inner: Arc<inner::Client>,
}

impl Client {
    /// Create a new client with the specified project_id, dataset, and api version.
    ///
    pub fn new(project: &str, dataset: &str, api: &str) -> Client {
        let client = reqwest::Client::new();
        let new_inner = inner::Client::new(client, project, dataset, api);
        Client {
            inner: Arc::new(new_inner),
        }
    }
    /// Set the authentication token
    ///
    /// [https://www.sanity.io/docs/http-auth]
    #[must_use]
    pub fn with_token(mut self, token: &str) -> Self {
        let mut new_inner = (*self.inner).clone();
        new_inner.set_token(token);
        self.inner = Arc::new(new_inner);
        self
    }
    /// Set or unset use of the API CDN.
    ///
    /// [https://www.sanity.io/docs/api-cdn]
    #[must_use]
    pub fn use_cdn(mut self, use_cdn: bool) -> Self {
        let mut new_inner = (*self.inner).clone();
        new_inner.set_cdn(use_cdn);
        self.inner = Arc::new(new_inner);
        self
    }
    /// Create a QueryBuilder to execute queries on the 'Query' endpoint.
    ///
    /// [https://www.sanity.io/docs/http-query]
    pub fn query<'a>(&self, query: &'a str) -> endpoints::QueryBuilder<'a> {
        endpoints::QueryBuilder::new(self.inner.clone(), query)
    }
    /// Create a DocumentsBuilder to query documents by id from the 'Doc' endpoint.
    ///
    /// [https://www.sanity.io/docs/http-doc]
    pub fn documents<'a>(&self, ids: &[&'a str]) -> endpoints::DocumentsBuilder<'a> {
        endpoints::DocumentsBuilder::new(self.inner.clone(), ids)
    }
    pub fn document<'a>(&self, id: &'a str) -> endpoints::DocumentsBuilder<'a> {
        endpoints::DocumentsBuilder::new(self.inner.clone(), &[id])
    }
}
