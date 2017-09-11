#![feature(conservative_impl_trait)]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate rand;

mod errors;
mod protocol;
mod client;

use std::io;
use rand::Rng;

use errors::*;
use protocol::*;
use client::HanabiClient;

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
    let mut client = {
        let uri = "http://localhost:9001".parse().chain_err(|| "cannot parse url")?;
        HanabiClient::new(uri)
    };
    let game_name = random_game_name();

    let req = StartGameRequest {
        num_players: 2,
        name: game_name.clone(),
    };
    let res = client.start_game(&req)?;
    println!("{:?}", res);

    let res = client.join_game(&JoinGameRequest{
        game_name: game_name.clone(),
        player_name: "player1".to_owned(),
    })?;
    println!("{:?}", res);
    let session = res.session;

    let res = client.get_state(&GetStateRequest{
        session: session,
        wait: false,
    })?;
    println!("{:?}", res);

    Ok(())
}

fn random_game_name() -> String {
    let mut rng = rand::thread_rng();
    let bs = rng.gen::<[u8; 8]>();
    let s: String = bs.iter().map(|b| format!("{}", b)).collect();
    format!("game-{}", s)
}
