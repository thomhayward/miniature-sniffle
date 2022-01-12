use super::SanityClientInner;
use reqwest::Method;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct QueryBuilder<'a> {
    client: Arc<SanityClientInner>,
    query: String,
    params: HashMap<&'a str, &'a str>,
}

impl<'a> QueryBuilder<'a> {
    pub(crate) fn new(client: Arc<SanityClientInner>, query: &str) -> QueryBuilder {
        QueryBuilder {
            client,
            query: String::from(query),
            params: HashMap::new(),
        }
    }
    //
    pub fn param(&'a mut self, k: &'a str, v: &'a str) -> &'a mut QueryBuilder {
        self.params.insert(k, v);
        self
    }
    //
    pub fn build(&self) -> reqwest::RequestBuilder {
        self.client
            .begin(Method::GET, "query", "")
            .query(&[("query", &self.query)])
            .query(&self.params)
    }
    //
    pub fn send(
        &self,
    ) -> impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>> {
        self.build().send()
    }
}
