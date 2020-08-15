#[macro_use]
extern crate rouille;

use rouille::Response;

use std::env;
use std::fs::File;
use std::sync::{Arc, Mutex};

mod game;

use game::{accept_ws, Game};

fn serve_html(path: &str) -> Response {
    let file = File::open(path);

    match file {
        Ok(file) => Response::from_file("html", file),
        Err(_) => Response::empty_404(),
    }
}

fn main() {
    let address = env::var("VOIN_BIND_TO").expect("Set `VOIN_BIND_TO` to specify a bind address");

    println!("Hosting on {}", address);

    let voin_game = Arc::new(Mutex::new(Game::new()));

    rouille::start_server(address, move |request| {
        router!(
            request,
            (GET) (/) => {
                serve_html("html/index.html")
            },
            (GET) (/ws) => {
                accept_ws(voin_game.clone(), request)
            },
            _ => {
                let response = rouille::match_assets(request, "static/");

                if response.is_success() {
                    response
                } else {
                    Response::empty_404()
                }
            },
        )
    });
}
