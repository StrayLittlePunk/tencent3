use std::time::Duration;

use hyper::Request;
use hyper::{client::HttpConnector, Body, Client, Method, Response};
use hyper_rustls::HttpsConnector;

use crate::api::TranslateMethods;

pub struct TencentClient<S> {
    pub client: Client<S>,
    pub credential: Credential,
    pub user_agent: String,
}

pub struct Credential {
    pub id: String,
    pub key: String,
}

impl<'a, S> TencentClient<S> {
    pub fn new(client: Client<S>, credential: Credential) -> Self {
        Self {
            client,
            credential,
            user_agent: r#"Mozilla/5.0 Safari/537.36"#.to_string(),
        }
    }
    /// Tencent Machine Translate APIs
    pub fn translate(&'a self) -> TranslateMethods<'a, S> {
        TranslateMethods { client: self }
    }
}

impl TencentClient<HttpsConnector<HttpConnector>> {
    /// construct HyperClient with no proxy
    pub fn native(credential: Credential) -> Self {
        let tls_connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();
        let client = Client::builder().build(tls_connector);
        Self::new(client, credential)
    }
}

/// A trait specifying functionality to help controlling any request performed by the API.
/// The trait has a conservative default implementation.
///
/// It contains methods to deal with all common issues
pub trait Delegate: Send {
    /// Called at the beginning of any API request. The delegate should store the method
    /// information if he is interesting in knowing more context when further calls to it
    /// are made.
    /// The matching `finished()` call will always be made, no matter whether or not the API
    /// request was successful. That way, the delegate may easily maintain a clean state
    /// between various API calls.
    fn begin(&mut self, _info: MethodInfo) {}

    /// Called whenever the http request returns with a non-success status code.
    /// The delegate should check the status, header to decide
    /// whether to retry or not. In the latter case, the underlying call will fail.
    ///
    /// If you choose to retry after a duration, the duration should be chosen using the
    /// [exponential backoff algorithm](http://en.wikipedia.org/wiki/Exponential_backoff).
    fn http_failure(&mut self, _: &Response<Body>) -> Retry {
        Retry::Abort
    }

    /// Called whenever there is an [HttpError](hyper::Error), usually if there are network problems.
    ///
    /// If you choose to retry after a duration, the duration should be chosen using the
    /// [exponential backoff algorithm](http://en.wikipedia.org/wiki/Exponential_backoff).
    ///
    /// Return retry information.
    fn http_error(&mut self, _err: &hyper::Error) -> Retry {
        Retry::Abort
    }

    /// Called prior to sending the main request of the given method. It can be used to time
    /// the call or to print progress information.
    /// It's also useful as you can be sure that a request will definitely be made.
    fn pre_request(&mut self, _request: &Request<Body>) {}

    /// retry times when http failure
    fn retry_times(&self) -> u8 {
        3
    }

    /// Called before the API request method returns, in every case. It can be used to clean up
    /// internal state between calls to the API.
    /// This call always has a matching call to `begin(...)`.
    ///
    /// # Arguments
    ///
    /// * `is_success` - a true value indicates the operation was successful.
    fn finished(&mut self, is_success: bool) {
        let _ = is_success;
    }
}

/// Contains information about an API request.
pub struct MethodInfo {
    pub id: &'static str,
    pub http_method: Method,
}

/// A delegate with a conservative default implementation, which is used if no other delegate is
/// set.
#[derive(Default)]
pub struct DefaultDelegate;

impl Delegate for DefaultDelegate {}

pub enum Retry {
    /// Signal you don't want to retry
    Abort,
    /// Signals you want to retry after the given duration
    After(Duration),
}
