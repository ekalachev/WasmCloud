use url::form_urlencoded;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
// use isahc::prelude::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct HelloActor {}

#[async_trait]
impl HttpServer for HelloActor {
    async fn handle_request(
        &self,
        _ctx: &Context,
        req: &HttpRequest,
    ) -> Result<HttpResponse, RpcError> {
        handle_http_request(req)
    }
}

fn handle_http_request(req: &HttpRequest) -> Result<HttpResponse, RpcError> {
    let hash = form_urlencoded::parse(req.query_string.as_bytes())
        .find(|(n, _)| n == "hash")
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "World".to_string());

    Ok(HttpResponse {
        body: hash.as_bytes().to_vec(),
        ..Default::default()
    })


    // let response_result = isahc::get(format!("https://ipfs.io/ipfs/{}", hash));

    // match response_result {
    //     Ok(mut response) => {
    //         let text_result = response.text();
    //         match text_result {
    //             Ok(text) => {
    //                 Ok(HttpResponse {
    //                     body: text.as_bytes().to_vec(),
    //                     ..Default::default()
    //                 })
    //             }
    //             Err(_) => {
    //                 Err(RpcError::from("Failed to read response text"))
    //             }
    //         }
    //     }
    //     Err(_) => {
    //         Err(RpcError::from("Failed to send request"))
    //     }
    // }
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
//             query_string: "hash=QmPtSHc2rNzqMNGCMAGibGXFaT75QeVWDdKqPWrVzc4fNL".to_string(),
//             header: HashMap::new(),
//             body: vec![],
//         };
//
//         let response = handle_http_request(&request).unwrap();
//
//         assert_eq!(response.status_code, 200);
//         assert_eq!(
//             String::from_utf8(response.body).unwrap(),
//             "Some test text here.".to_string()
//         );
//     }
// }
