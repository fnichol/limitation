// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use actix_http::{encoding::Decoder, Response};
use actix_web::{
    client::{Client, ClientRequest, ClientResponse},
    dev::Payload,
    http::{HeaderName, HeaderValue, Uri},
    web, Error, HttpRequest, HttpResponse,
};
use futures::Future;
use url::Url;

/// A list of "hop-by-hop" headers that should be removed when transparently proxying traffic.
///
/// For more details, see: https://www.w3.org/Protocols/rfc2616/rfc2616-sec13.html (Section
/// 13.5.1).
const HOP_BY_HOP_HEADERS: &[&str] = &[
    "Connection",
    "Keep-Alive",
    "Proxy-Authenticate",
    "Proxy-Authorization",
    "TE",
    "Trailers",
    "Transfer-Encoding",
    "Upgrade",
];

/// Forwards a request to a proxied backend and returns the proxied response.
pub fn forward(
    req: HttpRequest,
    payload: web::Payload,
    proxy_to: web::Data<Url>,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    proxied_request(&client, &proxy_to, &req)
        .send_stream(payload)
        .map_err(Error::from)
        .map(response)
}

/// Builds a proxied `ClientRequest` from the incoming `HttpRequest`.
fn proxied_request(client: &Client, proxy_to: &Url, req: &HttpRequest) -> ClientRequest {
    let mut proxied_req = client
        .request_from(proxy_url(&proxy_to, req.uri()).as_str(), req.head())
        .no_decompress();
    drop_hop_headers_on_request(&mut proxied_req);
    add_headers_on_request(&mut proxied_req, &req);

    proxied_req
}

/// Builds a `Response` from an incoming proxied `ClientResponse`.
fn response(proxied_response: ClientResponse<Decoder<Payload>>) -> Response {
    let headers = proxied_response.headers().clone();
    let mut res = HttpResponse::build(proxied_response.status()).streaming(proxied_response);
    res.head_mut().headers = headers;
    drop_hop_headers_on_response(&mut res);

    res
}

/// Drops "hop-by-hop" headers on the proxied `ClientRequest`.
fn drop_hop_headers_on_request(request: &mut ClientRequest) {
    let headers = request.headers_mut();

    for name in HOP_BY_HOP_HEADERS {
        headers.remove(*name);
    }
}

/// Add headers to the proxied `ClientRequest`.
fn add_headers_on_request(proxied_req: &mut ClientRequest, req: &HttpRequest) {
    if let Some(addr) = req.head().peer_addr {
        proxied_req.headers_mut().append(
            HeaderName::from_static("x-forwarded-for"),
            HeaderValue::from_str(&addr.ip().to_string()).expect("TODO: fix me"),
        );
    }
}

/// Drops "hop-by-hop" headers on the `HttpResponse`.
fn drop_hop_headers_on_response<B>(response: &mut HttpResponse<B>) {
    let headers = response.headers_mut();

    for name in HOP_BY_HOP_HEADERS {
        headers.remove(*name);
    }
}

/// Builds a proxied target `Url` from a request `Uri`.
fn proxy_url(proxy_to: &Url, uri: &Uri) -> Url {
    let mut url = proxy_to.clone();
    url.set_path(uri.path());
    url.set_query(uri.query());
    url
}
