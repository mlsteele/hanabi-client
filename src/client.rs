use hyper;
use serde_json;
use futures::{Future, Stream};
use tokio_core::reactor::Core;

use errors::*;
use protocol::*;

/// Client to the hanabi server
pub struct HanabiClient {
    uri: hyper::Uri,
}

impl HanabiClient {
    pub fn new(uri: hyper::Uri) -> Self {
        Self {
            uri: uri,
        }
    }

    pub fn start_game(&mut self, req: &StartGameRequest) -> Result<StartGameResponse> {
        let mut core = Core::new()?;
        let path = "hanabi/start-game";
        let uri = format!("{}/{}", self.uri, path).parse().chain_err(|| "cannot parse url")?;
        let req_json = serde_json::to_string(&req).chain_err(|| "serializing request")?;
        let client = hyper::Client::new(&core.handle());
        let mut hreq: hyper::Request = hyper::Request::new(hyper::Method::Post, uri);
        hreq.set_body(req_json);
        let res = core.run(client.request(hreq).map_err(Error::from))?;
        println!("Response: {}", res.status());
        let body = core.run(res.body().concat2().map_err(Error::from))?;

        {
            // Check the common status and reason
            let decode = serde_json::from_slice::<GenericResponse>(&body).map_err(Error::from);
            match decode {
                Err(err) => {
                    if let Ok(s) = String::from_utf8(body.to_vec()) {
                        println!("Body: {}", s); 
                    }
                    return Err(err)
                }
                Ok(gres) => {
                    if gres.status != "ok" {
                        if let Some(reason) = gres.reason {
                            return Err(Error::from(format!("error from server: {}", reason)))
                        } else {
                            return Err(Error::from("error with no reason given"))
                        }
                    }
                }
            }
        }

        // Decode the specific endpoint response.
        let decode = serde_json::from_slice::<StartGameResponse>(&body).map_err(Error::from);
        match decode {
            Err(err) => {
                if let Ok(s) = String::from_utf8(body.to_vec()) {
                    println!("Body: {}", s); 
                }
                Err(err)
            }
            ok @ Ok(_) => ok,
        }
    }

    pub fn join_game(&mut self, req: &JoinGameRequest) -> Result<JoinGameResponse> {
        let mut core = Core::new()?;
        let path = "hanabi/start-game";
        let uri = format!("{}/{}", self.uri, path).parse().chain_err(|| "cannot parse url")?;
        let req_json = serde_json::to_string(&req).chain_err(|| "serializing request")?;
        let client = hyper::Client::new(&core.handle());
        let mut hreq: hyper::Request = hyper::Request::new(hyper::Method::Post, uri);
        hreq.set_body(req_json);
        let res = core.run(client.request(hreq).map_err(Error::from))?;
        println!("Response: {}", res.status());
        let body = core.run(res.body().concat2().map_err(Error::from))?;

        {
            // Check the common status and reason
            let decode = serde_json::from_slice::<GenericResponse>(&body).map_err(Error::from);
            match decode {
                Err(err) => {
                    if let Ok(s) = String::from_utf8(body.to_vec()) {
                        println!("Body: {}", s); 
                    }
                    return Err(err)
                }
                Ok(gres) => {
                    if gres.status != "ok" {
                        if let Some(reason) = gres.reason {
                            return Err(Error::from(format!("error from server: {}", reason)))
                        } else {
                            return Err(Error::from("error with no reason given"))
                        }
                    }
                }
            }
        }

        // Decode the specific endpoint response.
        let decode = serde_json::from_slice::<JoinGameResponse>(&body).map_err(Error::from);
        match decode {
            Err(err) => {
                if let Ok(s) = String::from_utf8(body.to_vec()) {
                    println!("Body: {}", s); 
                }
                Err(err)
            }
            ok @ Ok(_) => ok,
        }
    }

}
