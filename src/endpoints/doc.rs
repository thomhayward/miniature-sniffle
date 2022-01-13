use crate::inner::Client;
use reqwest::Method;
use std::sync::Arc;

pub struct DocBuilder<'a> {
    client: Arc<Client>,
    document_ids: Vec<&'a str>,
}

impl<'a> DocBuilder<'a> {
    pub(crate) fn new(client: Arc<Client>, ids: &[&'a str]) -> DocBuilder<'a> {
        DocBuilder {
            client,
            document_ids: Vec::from(ids),
        }
    }
    pub fn documents(&'a mut self, ids: &[&'a str]) -> &'a mut DocBuilder {
        self.document_ids.extend_from_slice(ids);
        self
    }
    /// Add a document id to fetch to the builder
    ///
    pub fn document(&'a mut self, id: &'a str) -> &'a mut DocBuilder {
        self.document_ids.push(id);
        self
    }
    //
    pub fn build(&self) -> reqwest::RequestBuilder {
        self.client
            .begin(Method::GET, "doc", &self.document_ids.join(","))
    }
    //
    pub fn send(
        &self,
    ) -> impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>> {
        self.build().send()
    }
}
