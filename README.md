# Overview

A Rusty Tencent Api Client with v3 authorization. Only machine translation api is supported now, other apis are not supported at present

## Example

```rust
fn build_client() -> TencentClient<HttpsConnector<HttpConnector>> {
    let client = TencentClient::native(client::Credential {
        key: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
        id: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
    });
    client
}

fn main()  {
    let client = build_client();
    let call = client
        .translate()
        .text_translate()
        .source("it") // Italy
        .target("zh")
        .project_id(PROJECT_ID)
        .region("REGION")
        .source_text("Credere è destino")
        .build()
        .unwrap();
        // {"Response":{"RequestId":"38b2df48-48e6-4aa5-ace4-xxxxxxxxx","Source":"it","Target":"zh","TargetText":"相信就是命运"}}
    let result = call
        .doit(|body| {
            let string = String::from_utf8(body).unwrap();
            let value = serde_json::from_str::<serde_json::Value>(&string).unwrap();
            let text = value
                .get("Response")
                .and_then(|res| res.get("TargetText"))
                .and_then(|e| e.as_str())
                .unwrap();
            assert_eq!(text, "相信就是命运");
         })
         .await;
}
```

## The API is structured into the following primary items:

### Client
   - a central object to maintain state and allow accessing all Activities
   - creates Method Builders which in turn allow access to individual Call Builders

### Resources
   - primary types that you can apply Activities to
   - a collection of properties and Parts

### Parts
   - a collection of properties never directly used in Activities

### Activities
   - operations to apply to Resources