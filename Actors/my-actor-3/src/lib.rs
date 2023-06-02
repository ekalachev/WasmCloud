use url::form_urlencoded;
use wasmbus_rpc::actor::prelude::*;

use wasmcloud_interface_httpserver::{HttpRequest as ServerHttpRequest, HttpResponse as ServerHttpResponse, HttpServer as ServerHttpServer, HttpServerReceiver};
use wasmcloud_interface_httpclient::{HttpClient, HttpClientSender, HttpRequest};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct MyActor3Actor {}

#[async_trait]
impl ServerHttpServer for MyActor3Actor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &ServerHttpRequest,
    ) -> Result<ServerHttpResponse, RpcError> {
        handle_http_request(ctx, req).await
    }
}

async fn handle_http_request(ctx: &Context, req: &ServerHttpRequest) -> RpcResult<ServerHttpResponse> {
    let client = HttpClientSender::new();

    let hash = get_hash(req);

    // The IPFS URI to the wasm binary
    let uri = format!("https://ipfs.io/ipfs/{}?filename=index.html", hash);

    // Construct the HttpClientSender and issue the request
    let req = HttpRequest::get(&uri);
    let response = client.request(ctx, &req).await?;

    Ok(ServerHttpResponse::ok(response.body.clone()))
}

fn get_hash(req: &ServerHttpRequest) -> String {
    let mut query = form_urlencoded::parse(req.query_string.as_bytes());
    let hash = query
        .find(|(n, _)| n == "hash")
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "111".to_string());
    hash
}


#[cfg(test)]
mod test {
    use super::handle_http_request;
    use std::collections::HashMap;
    use wasmbus_rpc::common::Context;
    use wasmcloud_interface_httpserver::{HttpRequest};

    use futures::executor::block_on;

    #[test]
    fn can_handle_request() {
        let request = HttpRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            query_string: "hash=QmUcuakCUoTCQQ28Dx3yToziRhPuWAeh11K1btKuYzyWSF".to_string(),
            header: HashMap::new(),
            body: vec![],
        };

        let ctx = Context::default(); // Create a context

        let response = block_on(handle_http_request(&ctx, &request));

        assert!(response.is_ok());
        let response = response.unwrap();

        assert_eq!(response.status_code, 200);
        // assert_eq!(
        //     String::from_utf8(response.body).unwrap(),
        //     "Hello, Eldar!\n".to_string()
        // );
    }
}
