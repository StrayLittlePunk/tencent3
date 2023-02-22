use std::path::PathBuf;

use hyper::{
    body::{self, Buf},
    client::connect::Connection,
    header::{AUTHORIZATION, CONTENT_TYPE, HOST, USER_AGENT},
    http::request::Builder,
    service::Service,
    Body, Method, Request, Uri,
};
use serde::Serialize;
use tokio::io::{AsyncRead, AsyncWrite};

use super::{
    utils::{signature_v3_with_post, to_base64, SignatureV3Arg},
    CallOutput, JSON_MIME,
};
use crate::{
    client::{self, Delegate},
    Error, Result, TencentClient,
};

const API_VERSION: &str = "2018-03-21";
const BASE_URL: &str = "https://tmt.tencentcloudapi.com/";
const BASE_HOST: &str = "tmt.tencentcloudapi.com";
const SERVICE: &str = "tmt";

pub struct TranslateMethods<'a, S>
where
    S: 'a,
{
    pub client: &'a TencentClient<S>,
}

impl<'a, S> TranslateMethods<'a, S> {
    /// Create builder to help you perform the following task:
    /// translate a file(resource)
    pub fn file_translate(&self) -> FileTranslateCallBuilder<'a, S> {
        FileTranslateCallBuilder::default().client(self.client)
    }

    /// Create builder to help you perform the following task:
    /// translate a file(resource)
    pub fn get_file_translate_data(&self) -> FileTranslateDataCallBuilder<'a, S> {
        FileTranslateDataCallBuilder::default().client(self.client)
    }

    /// Create builder to help you perform the following task:
    /// translate a picture(resource)
    pub fn image_translate(&self) -> ImageTranslateCallBuilder<'a, S> {
        ImageTranslateCallBuilder::default().client(self.client)
    }

    // Create builder to help you perform the following task:
    /// detect text to identify which language
    pub fn language_detect(&self) -> LanguageDetectCallBuilder<'a, S> {
        LanguageDetectCallBuilder::default().client(self.client)
    }

    /// Create builder to help you perform the following task:
    /// detect text to identify which language
    pub fn speech_translate(&self) -> SpeechTranslateCallBuilder<'a, S> {
        SpeechTranslateCallBuilder::default().client(self.client)
    }

    /// Create builder to help you perform the following task:
    /// translate text
    pub fn text_translate(&self) -> TextTranslateCallBuilder<'a, S> {
        TextTranslateCallBuilder::default().client(self.client)
    }
    /// Create builder to help you perform the following task:
    /// translate text
    pub fn text_batch_translate(&self) -> TextTranslateBatchCallBuilder<'a, S> {
        TextTranslateBatchCallBuilder::default().client(self.client)
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct FileTranslateDataCall<'a, S>
where
    S: 'a,
{
    client: &'a TencentClient<S>,
    #[builder(setter(into))]
    task_id: String,
    #[builder(setter(strip_option), default)]
    delegate: Option<&'a mut dyn Delegate>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct FileTranslateDataPayload {
    task_id: String,
}

impl<'a, S> FileTranslateDataCall<'a, S>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    pub async fn doit<O, F>(self, mut f: F) -> Result<O>
    where
        O: CallOutput,
        F: FnMut(Vec<u8>) -> O,
    {
        let payload = FileTranslateDataPayload {
            task_id: self.task_id,
        };
        let request_payload = serde_json::to_string(&payload)
            .map_err(|e| Error::JsonError(format!("{payload:?}"), e))?;

        let arg = DoitArg {
            request_payload,
            action: "GetFileTranslate",
            dlg: self.delegate,
            client: self.client,
            doid: "tmt.getFileTranslateData",
        };
        Ok(f(doit(arg, |b| b).await?))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct FileTranslatePayload {
    source: String,
    target: String,
    document_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_document_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct FileTranslateCall<'a, S>
where
    S: 'a,
{
    client: &'a TencentClient<S>,
    #[builder(setter(into))]
    source: String,
    #[builder(setter(into))]
    target: String,
    #[builder(setter(into))]
    document_type: String,
    #[builder(setter(into, strip_option), default)]
    source_type: Option<u8>,
    #[builder(setter(into, strip_option), default)]
    basic_document_type: Option<String>,
    #[builder(setter(into, strip_option), default)]
    callback_url: Option<String>,
    #[builder(setter(into, strip_option), default)]
    url: Option<String>,
    #[builder(setter(into, strip_option), default)]
    data: Option<String>,
    #[builder(setter(strip_option), default)]
    delegate: Option<&'a mut dyn Delegate>,
}

impl<'a, S> FileTranslateCall<'a, S>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    pub async fn doit<O, F>(self, mut f: F) -> Result<O>
    where
        O: CallOutput,
        F: FnMut(Vec<u8>) -> O,
    {
        let payload = FileTranslatePayload {
            source: self.source,
            target: self.target,
            document_type: self.document_type,
            basic_document_type: self.basic_document_type,
            source_type: self.source_type,
            url: self.url,
            callback_url: self.callback_url,
            data: self.data,
        };
        let request_payload = serde_json::to_string(&payload)
            .map_err(|e| Error::JsonError(format!("{payload:?}"), e))?;

        let arg = DoitArg {
            request_payload,
            action: "FileTranslate",
            dlg: self.delegate,
            client: self.client,
            doid: "tmt.FileTranslate",
        };

        Ok(f(doit(arg, |b| b).await?))
    }
}

// project id 1283783
#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct ImageTranslateCall<'a, S>
where
    S: 'a,
{
    client: &'a TencentClient<S>,
    project_id: u32,
    #[builder(setter(into))]
    source: String,
    #[builder(setter(into))]
    target: String,
    #[builder(setter(into))]
    session_uuid: String,
    #[builder(setter(into))]
    scene: String,
    #[builder(setter(into))]
    image_path: PathBuf,
    #[builder(setter(into))]
    region: String,
    #[builder(setter(strip_option), default)]
    delegate: Option<&'a mut dyn Delegate>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageTranslatePayload {
    project_id: u32,
    source: String,
    target: String,
    session_uuid: String,
    scene: String,
    data: String,
}

impl<'a, S> ImageTranslateCall<'a, S>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    pub async fn doit<O, F>(self, mut f: F) -> Result<O>
    where
        O: CallOutput,
        F: FnMut(Vec<u8>) -> O,
    {
        let metadata = tokio::fs::metadata(self.image_path.as_path()).await?;
        // 图片大小上限为4M，建议对源图片进行一定程度压缩
        if metadata.len() >= 4 << 20 {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "image size should be no more than 4M",
            )));
        }
        let data = tokio::fs::read(self.image_path).await?;
        let payload = ImageTranslatePayload {
            source: self.source,
            target: self.target,
            data: to_base64(data),
            project_id: self.project_id,
            scene: self.scene,
            session_uuid: self.session_uuid,
        };

        let request_payload = serde_json::to_string(&payload)
            .map_err(|e| Error::JsonError(format!("{payload:?}"), e))?;

        let arg = DoitArg {
            request_payload,
            action: "ImageTranslate",
            dlg: self.delegate,
            client: self.client,
            doid: "tmt.ImageTranslate",
        };

        let b = |builder: Builder| builder.header("X-TC-Region", self.region.clone());
        Ok(f(doit(arg, b).await?))
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct LanguageDetectCall<'a, S>
where
    S: 'a,
{
    client: &'a TencentClient<S>,
    #[builder(setter(strip_option), default)]
    delegate: Option<&'a mut dyn Delegate>,
    #[builder(setter(into))]
    region: String,
    project_id: u32,
    #[builder(setter(into))]
    text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LanguageDetectPayload {
    project_id: u32,
    text: String,
}

impl<'a, S> LanguageDetectCall<'a, S>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    pub async fn doit<O, F>(self, mut f: F) -> Result<O>
    where
        O: CallOutput,
        F: FnMut(Vec<u8>) -> O,
    {
        let payload = LanguageDetectPayload {
            text: self.text,
            project_id: self.project_id,
        };

        let request_payload = serde_json::to_string(&payload)
            .map_err(|e| Error::JsonError(format!("{payload:?}"), e))?;

        let arg = DoitArg {
            request_payload,
            action: "LanguageDetect",
            dlg: self.delegate,
            client: self.client,
            doid: "tmt.LanguageDetect",
        };

        let b = |builder: Builder| builder.header("X-TC-Region", self.region.clone());
        Ok(f(doit(arg, b).await?))
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct SpeechTranslateCall<'a, S>
where
    S: 'a,
{
    client: &'a TencentClient<S>,
    #[builder(setter(strip_option), default)]
    project_id: Option<u32>,
    #[builder(setter(into))]
    source: String,
    #[builder(setter(into))]
    target: String,
    #[builder(setter(into))]
    session_uuid: String,
    #[builder(setter(into))]
    audio_path: PathBuf,
    #[builder(setter(into))]
    region: String,
    audio_format: u32,
    seq: u32,
    is_end: u8,
    #[builder(setter(strip_option), default)]
    delegate: Option<&'a mut dyn Delegate>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpeechTranslatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<u32>,
    source: String,
    target: String,
    session_uuid: String,
    data: String,
    audio_format: u32,
    seq: u32,
    is_end: u8,
}

impl<'a, S> SpeechTranslateCall<'a, S>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    pub async fn doit<O, F>(self, mut f: F) -> Result<O>
    where
        O: CallOutput,
        F: FnMut(Vec<u8>) -> O,
    {
        let metadata = tokio::fs::metadata(self.audio_path.as_path()).await?;
        // 暂时也认为声音大小上限为4M
        if metadata.len() >= 4 << 20 {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "audio file size should be no more than 4M",
            )));
        }
        let data = tokio::fs::read(self.audio_path).await?;
        let payload = SpeechTranslatePayload {
            source: self.source,
            target: self.target,
            data: to_base64(data),
            project_id: self.project_id,
            session_uuid: self.session_uuid,
            is_end: self.is_end,
            audio_format: self.audio_format,
            seq: self.seq,
        };

        let request_payload = serde_json::to_string(&payload)
            .map_err(|e| Error::JsonError(format!("{payload:?}"), e))?;

        let arg = DoitArg {
            request_payload,
            action: "SpeechTranslate",
            dlg: self.delegate,
            client: self.client,
            doid: "tmt.SpeechTranslate",
        };

        let b = |builder: Builder| builder.header("X-TC-Region", self.region.clone());
        Ok(f(doit(arg, b).await?))
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct TextTranslateCall<'a, S>
where
    S: 'a,
{
    client: &'a TencentClient<S>,
    #[builder(setter(strip_option), default)]
    delegate: Option<&'a mut dyn Delegate>,
    project_id: u32,
    #[builder(setter(into))]
    source: String,
    #[builder(setter(into))]
    target: String,
    #[builder(setter(into))]
    region: String,
    #[builder(setter(into))]
    source_text: String,
    #[builder(setter(into, strip_option), default)]
    untranslated_text: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextTranslatePayload {
    project_id: u32,
    source: String,
    target: String,
    source_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    untranslated_text: Option<String>,
}

impl<'a, S> TextTranslateCall<'a, S>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    pub async fn doit<O, F>(self, mut f: F) -> Result<O>
    where
        O: CallOutput,
        F: FnMut(Vec<u8>) -> O,
    {
        let payload = TextTranslatePayload {
            source: self.source,
            target: self.target,
            project_id: self.project_id,
            source_text: self.source_text,
            untranslated_text: self.untranslated_text,
        };

        let request_payload = serde_json::to_string(&payload)
            .map_err(|e| Error::JsonError(format!("{payload:?}"), e))?;

        let arg = DoitArg {
            request_payload,
            action: "TextTranslate",
            dlg: self.delegate,
            client: self.client,
            doid: "tmt.TextTranslate",
        };

        let b = |builder: Builder| builder.header("X-TC-Region", self.region.clone());
        Ok(f(doit(arg, b).await?))
    }
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct TextTranslateBatchCall<'a, S>
where
    S: 'a,
{
    client: &'a TencentClient<S>,
    #[builder(setter(strip_option), default)]
    delegate: Option<&'a mut dyn Delegate>,
    project_id: u32,
    #[builder(setter(into))]
    source: String,
    #[builder(setter(into))]
    target: String,
    #[builder(setter(into))]
    region: String,
    #[builder(setter(into))]
    source_text_list: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextTranslateBatchPayload {
    project_id: u32,
    source: String,
    target: String,
    source_text_list: Vec<String>,
}

impl<'a, S> TextTranslateBatchCall<'a, S>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    pub async fn doit<O, F>(self, mut f: F) -> Result<O>
    where
        O: CallOutput,
        F: FnMut(Vec<u8>) -> O,
    {
        let payload = TextTranslateBatchPayload {
            source: self.source,
            target: self.target,
            project_id: self.project_id,
            source_text_list: self.source_text_list,
        };

        let request_payload = serde_json::to_string(&payload)
            .map_err(|e| Error::JsonError(format!("{payload:?}"), e))?;

        let arg = DoitArg {
            request_payload,
            action: "TextTranslateBatch",
            dlg: self.delegate,
            client: self.client,
            doid: "tmt.TextTranslateBatch",
        };

        let b = |builder: Builder| builder.header("X-TC-Region", self.region.clone());
        Ok(f(doit(arg, b).await?))
    }
}

impl CallOutput for () {}
impl CallOutput for String {}
impl CallOutput for Vec<String> {}

struct DoitArg<'a, S>
where
    S: 'a,
{
    request_payload: String,
    client: &'a TencentClient<S>,
    dlg: Option<&'a mut dyn Delegate>,
    action: &'static str,
    doid: &'static str,
}

async fn doit<S, F>(arg: DoitArg<'_, S>, f: F) -> Result<Vec<u8>>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    F: Fn(Builder) -> Builder,
{
    let DoitArg {
        request_payload,
        dlg: delegate,
        action,
        client,
        doid,
    } = arg;

    let mut dd = client::DefaultDelegate;
    let dlg: &mut dyn client::Delegate = match delegate {
        Some(d) => d,
        None => &mut dd,
    };
    dlg.begin(client::MethodInfo {
        id: doid,
        http_method: Method::POST,
    });

    let retry_times = dlg.retry_times() as usize;
    for i in 0..retry_times {
        let req_result = {
            let https_client = &client.client;
            let timestamp = chrono::Utc::now().timestamp();
            let mut req_builder = Request::builder()
                .method(Method::POST)
                .uri(BASE_URL)
                .header(USER_AGENT, client.user_agent.as_str())
                .header(CONTENT_TYPE, JSON_MIME)
                .header(HOST, BASE_HOST)
                .header("X-TC-Action", action)
                .header("X-TC-Timestamp", timestamp)
                .header("X-TC-Language", "zh-CN")
                .header("X-TC-RequestClient", "rust-sdk")
                .header("X-TC-Version", API_VERSION);
            // custom construct request header
            req_builder = f(req_builder);

            let arg = SignatureV3Arg {
                content_type: JSON_MIME,
                host: BASE_HOST,
                service: SERVICE,
                secret_key: &client.credential.key,
                secret_id: &client.credential.id,
                request_payload: &request_payload,
                timestamp: timestamp as u64,
            };
            req_builder = req_builder.header(AUTHORIZATION, signature_v3_with_post(arg));

            let request = req_builder
                .body(Body::from(request_payload.clone()))
                .unwrap();
            dlg.pre_request(&request);
            https_client.request(request).await
        };

        match req_result {
            Err(err) => {
                if let client::Retry::After(d) = dlg.http_error(&err) {
                    // last request should not sleep
                    if i + 1 == retry_times {
                        break;
                    }
                    tokio::time::sleep(d).await;
                    continue;
                }
                dlg.finished(false);
                return Err(Error::HttpError(err));
            }
            Ok(res) => {
                if !res.status().is_success() {
                    if let client::Retry::After(d) = dlg.http_failure(&res) {
                        // last request should not sleep
                        if i + 1 == retry_times {
                            break;
                        }
                        tokio::time::sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::Failure(res));
                }
                let mut bytes = body::aggregate(res.into_body()).await.unwrap();
                let mut result = vec![0; bytes.remaining()];
                bytes.copy_to_slice(&mut result);
                return Ok(result);
            }
        }
    }
    Err(Error::Cancelled)
}
