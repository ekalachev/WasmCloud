use url::form_urlencoded;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::cbor::Type::Bytes;

use wasmcloud_interface_httpserver::{HttpRequest as ServerHttpRequest, HttpResponse as ServerHttpResponse, HttpServer as ServerHttpServer, HttpServerReceiver};
use wasmcloud_interface_httpclient::{HttpClient, HttpClientSender, HttpRequest};

// use wasmer::{Module, Store};
// use wasmer_wasix::{Pipe, WasiEnv};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct HelloActor {}

#[async_trait]
impl ServerHttpServer for HelloActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &ServerHttpRequest,
    ) -> Result<ServerHttpResponse, RpcError> {
        handle_http_request(ctx, req).await
    }
}

async fn handle_http_request(ctx: &Context, req: &ServerHttpRequest) -> RpcResult<ServerHttpResponse> {
    let mut query = form_urlencoded::parse(req.query_string.as_bytes());
    let hash = query
        .find(|(n, _)| n == "hash")
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "111".to_string());

    // The IPFS URI to the wasm binary
    let uri = format!("https://ipfs.io/ipfs/{}?filename=HelloWorld.wasm", hash);

    // Construct the HttpClientSender and issue the request
    let client = HttpClientSender::new();
    let req = HttpRequest::get(&uri);
    let response = client.request(ctx, &req).await?;

    // // Create a new wasm store
    // let mut store = Store::default();
    //
    // // Create a new wasm module from the binary
    // let module = match Module::new(&store, &response.body.clone()) {
    //     Ok(m) => m,
    //     Err(error) => panic!("Problem creating the module: {:?}", error),
    // };
    //
    // // Create a channel for the wasm module's stdout
    // let (stdout_tx, mut stdout_rx) = Pipe::channel();
    //
    // // Build the wasm environment and execute the module
    // let mut builder = WasiEnv::builder("hello")
    //     .stdout(Box::new(stdout_tx))
    //     .env("NAME", "Eldar");
    //
    // let execution_result = builder.run_with_store(module, &mut store);
    //
    // let mut buf = String::new();
    //
    // match execution_result {
    //     Ok(_) => {
    //         // Read the output of the wasm module
    //         stdout_rx.read_to_string(&mut buf).unwrap();
    //     },
    //     Err(error) => panic!("Problem executing the file: {:?}", error),
    // };

    Ok(ServerHttpResponse::ok(response))
}

// #[cfg(test)]
// mod test {
//     use crate::handle_http_request;
//     use std::collections::HashMap;
//     use wasmcloud_interface_httpserver::{HttpRequest};
//
//     #[test]
//     fn can_handle_request() {
//         let request = HttpRequest {
//             method: "GET".to_string(),
//             path: "/".to_string(),
//             query_string: "hash=QmUcuakCUoTCQQ28Dx3yToziRhPuWAeh11K1btKuYzyWSF&name=Eldar".to_string(),
//             header: HashMap::new(),
//             body: vec![],
//         };
//
//         let response = handle_http_request(&request).unwrap();
//
//         assert_eq!(response.status_code, 200);
//         assert_eq!(
//             String::from_utf8(response.body).unwrap(),
//             "Hello, Eldar!\n".to_string()
//         );
//     }
// }
