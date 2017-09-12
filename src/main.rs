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
use std::thread;
use rand::Rng;

use errors::*;
use protocol::*;
use protocol::GameState::*;
use client::HanabiClient;

fn main() {
    let res = main2();
    let is_err = res.is_err();
    report_err(res);
    if is_err {
        ::std::process::exit(1);
    }
}

fn report_err<T>(res: Result<T>) {
    if let Err(ref e) = res {
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

    let mut threads = vec![];
    let client2 = client.clone();
    let game_name2 = game_name.clone();
    threads.push(thread::spawn(move || {
        player_thread(client2, game_name2, 1);
    }));
    threads.push(thread::spawn(move || {
        player_thread(client.clone(), game_name.clone(), 2);
    }));

    for t in threads {
        t.join().unwrap();
    }

    println!("bye");
    Ok(())
}

fn player_thread(client: HanabiClient, game_name: String, player_number: i32) {
    let res = run1(client, game_name, player_number);
    report_err(res);
}

fn run1(mut client: HanabiClient, game_name: String, player_number: i32) -> Result<()> {
    let res = client.join_game(&JoinGameRequest{
        game_name: game_name.clone(),
        player_name: format!("player-{}", player_number),
    })?;
    println!("{:?}", res);
    let session = res.session;

    for _ in 0..10 {
        let res = client.get_state(&GetStateRequest{
            session: session.clone(),
            wait: true,
        })?;
        println!("state: {:?}", res.state.state);
        if res.state.state != YourTurn {
            return Err(Error::from("expected state to be your-turn"));
        }
    }

    println!("bye");
    Ok(())
}

fn random_game_name() -> String {
    let mut rng = rand::thread_rng();
    let bs = rng.gen::<[u8; 8]>();
    let s: String = bs.iter().map(|b| format!("{}", b)).collect();
    format!("game-{}", s)
}
