use crate::inner::Client;
use reqwest::{Method, Response};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub struct QueryBuilder<'a> {
    client: Arc<Client>,
    query: String,
    params: HashMap<&'a str, &'a str>,
}

impl<'a> QueryBuilder<'a> {
    pub(crate) fn new(client: Arc<Client>, query: &str) -> QueryBuilder {
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
    pub fn send(&self) -> impl Future<Output = Result<Response, reqwest::Error>> {
        self.build().send()
    }
    //
    #[cfg(feature = "json")]
    pub async fn json<T: DeserializeOwned>(self) -> Result<crate::ApiResponse<T>, reqwest::Error> {
        self.send().await?.json().await
    }
}
