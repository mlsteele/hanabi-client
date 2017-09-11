use hyper;
use tokio_core::reactor::Handle;
use serde_json;
use futures::{future, Future, Stream};
use io;

use errors::*;
use protocol::*;

/// Client to the hanabi server
pub struct HanabiClient {
    uri: hyper::Uri,
    handle: Handle,
}

impl HanabiClient {
    pub fn new(handle: Handle, uri: hyper::Uri) -> Self {
        Self {
            uri: uri,
            handle: handle,
        }
    }

    pub fn start_game(&mut self, req: &StartGameRequest) -> impl Future<Item=StartGameResponse, Error=Error> {
        future::ok(()).and_then(move |()| {
            let path = "hanabi/start-game";
            let uri_str = format!("{}/{}", self.uri, path);
            uri_str.parse().chain_err(|| "cannot parse url")
        }).and_then(move |uri: hyper::Uri| {
            serde_json::to_string(&req).chain_err(|| "serializing request")
                .map(move |req_json| (uri, req_json))
        }).and_then(move |(uri, req_json)| {
            let client = hyper::Client::new(&self.handle);
            let mut hreq: hyper::Request = hyper::Request::new(hyper::Method::Post, uri);
            hreq.set_body(req_json);
            client.request(hreq).map_err(Error::from)
        }).and_then(move |res| {
            println!("Response: {}", res.status());

            res.body().concat2().and_then(move |body| {
                let v: StartGameResponse = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, e)
                })?;
                Ok(v)
            }).map_err(Error::from)
        })
    }

    pub fn join_game(&mut self, req: &JoinGameRequest) -> impl Future<Item=JoinGameResponse, Error=Error> {
        let path = "hanabi/start-game";
        let uri = format!("{}/{}", self.uri, path);
        let client = hyper::Client::new(&self.handle);
        let mut hreq: hyper::Request = hyper::Request::new(hyper::Method::Post, uri);
        future::result(serde_json::to_string(&req).chain_err(|| "serializing request"))
        .and_then(move |req| {
            hreq.set_body(req);
            client.request(hreq).map_err(Error::from)
        }).and_then(move |res| {
            println!("Response: {}", res.status());
            res.body().concat2().map_err(Error::from)
        }).and_then(move |body| {
            // Check the common status and reason
            let decode = serde_json::from_slice::<GenericResponse>(&body).map_err(Error::from);
            decode.map_err(|err| {
                if let Ok(s) = String::from_utf8(body.to_vec()) {
                    println!("Body: {}", s); 
                }
                err
            }).and_then(|gres| {
                if gres.status == "ok" {
                    Ok(body)
                } else {
                    if let Some(reason) = gres.reason {
                        Err(Error::from(format!("error from server: {}", reason)))
                    } else {
                        Err(Error::from("error with no reason given"))
                    }
                }
            })
        }).and_then(move |body| {
            // Decode the specific endpoint response.
            let decode = serde_json::from_slice::<JoinGameResponse>(&body).map_err(Error::from);
            decode.map_err(|err| {
                if let Ok(s) = String::from_utf8(body.to_vec()) {
                    println!("Body: {}", s); 
                }
                err
            })
        })
    }
}
