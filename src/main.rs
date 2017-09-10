// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

mod errors;
mod protocol;

use std::io;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

use errors::*;
use protocol::*;

fn main() {
    if let Err(ref e) = main2() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let uri = "http://localhost:9001/hanabi/start-game".parse().chain_err(|| "cannot parse url")?;
    let req = StartGameRequest{
        num_players: 2,
        name: "ooh-new-game".to_owned(),
    };
    let mut hreq: hyper::Request = hyper::Request::new(hyper::Method::Post, uri);
    hreq.set_body(serde_json::to_string(&req)?);
    let work = client.request(hreq).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().concat2().and_then(move |body| {
            let v: StartGameResponse = serde_json::from_slice(&body).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    e
                )
            })?;
            println!("{:?}", v);
            Ok(())
        })
    });
    core.run(work)?;
    Ok(())
}
