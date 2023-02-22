//! # Overview
//! A Rusty Tencent Api Client with v3 authorization
//!
//! ## Example
//!```ignore
//! fn build_client() -> TencentClient<HttpsConnector<HttpConnector>> {
//!     let client = TencentClient::native(client::Credential {
//!         key: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
//!         id: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
//!     });
//!     client
//! }
//!
//! fn main()  {
//!     let client = build_client();
//!     let call = client
//!         .translate()
//!         .text_translate()
//!         .source("it") // Italy
//!         .target("zh")
//!         .project_id(PROJECT_ID)
//!         .region("REGION")
//!         .source_text("Credere è destino")
//!         .build()
//!         .unwrap();
//!         // {"Response":{"RequestId":"38b2df48-48e6-4aa5-ace4-xxxxxxxxx","Source":"it","Target":"zh","TargetText":"相信就是命运"}}
//!     let result = call
//!         .doit(|body| {
//!             let string = String::from_utf8(body).unwrap();
//!             let value = serde_json::from_str::<serde_json::Value>(&string).unwrap();
//!             let text = value
//!                 .get("Response")
//!                 .and_then(|res| res.get("TargetText"))
//!                 .and_then(|e| e.as_str())
//!                 .unwrap();
//!             assert_eq!(text, "相信就是命运");
//!          })
//!          .await;
//! }
//!```
//!
//! The API is structured into the following primary items:
//!
//! Client
//!    a central object to maintain state and allow accessing all Activities
//!    creates Method Builders which in turn allow access to individual Call Builders
//!
//! Resources
//!    primary types that you can apply Activities to
//!    a collection of properties and Parts
//!
//! Parts
//!    a collection of properties never directly used in Activities
//!
//! Activities
//!    operations to apply to Resources

pub mod api;
pub mod client;
pub use api::CallOutput;
pub use client::{Credential, TencentClient};

pub use hyper;
pub use hyper_rustls;

#[derive(Debug)]
pub enum Error {
    /// The http connection failed
    HttpError(hyper::Error),

    /// An attempt was made to upload a resource with size stored in field `.0`
    /// even though the maximum upload size is what is stored in field `.1`.
    UploadSizeLimitExceeded(u64, u64),

    /// Represents information about a request that was not understood by the server.
    /// Details are included.
    BadRequest(serde_json::Value),

    /// We needed an API key for authentication, but didn't obtain one.
    /// Neither through the authenticator, nor through the Delegate.
    MissingAPIKey,

    /// We required a Token, but didn't get one from the Authenticator
    //MissingToken(oauth2::Error),

    /// The delegate instructed to cancel the operation
    Cancelled,

    /// An additional, free form field clashed with one of the built-in optional ones
    FieldClash(&'static str),

    /// Missing field in CallBuilder
    MissingField(&'static str),

    /// Shows that we failed to encode/decode request/response.
    /// This can happen if the protocol changes in conjunction with strict json decoding.
    JsonError(String, serde_json::Error),

    /// Indicates an HTTP response with a non-success status code
    Failure(hyper::Response<hyper::body::Body>),

    /// An IO error occurred while reading a stream into memory
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::HttpError(ref err) => err.fmt(f),
            Error::UploadSizeLimitExceeded(ref resource_size, ref max_size) => writeln!(
                f,
                "The media size {} exceeds the maximum allowed upload size of {}",
                resource_size, max_size
            ),
            Error::MissingAPIKey => {
                (writeln!(
                    f,
                    "The application's API key was not found in the configuration"
                ))
                .ok();
                writeln!(
                    f,
                    "It is used as there are no Scopes defined for this method."
                )
            }
            Error::BadRequest(ref message) => {
                writeln!(f, "Bad Request: {}", message)?;
                Ok(())
            }
            Error::Cancelled => writeln!(f, "Operation cancelled by delegate"),
            Error::FieldClash(field) => writeln!(
                f,
                "The custom parameter '{}' is already provided natively by the CallBuilder.",
                field
            ),
            Error::MissingField(field) => writeln!(
                f,
                "The parameter '{}' is missing by the CallBuilder.",
                field
            ),
            Error::JsonError(ref json_str, ref err) => writeln!(f, "{}: {}", err, json_str),
            Error::Failure(ref response) => {
                writeln!(f, "Http status indicates failure: {:?}", response)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::HttpError(ref err) => err.source(),
            Error::JsonError(_, ref err) => err.source(),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
