#[macro_use]
extern crate rouille;

use rouille::Response;

use std::env;
use std::fs::File;

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

    rouille::start_server(address, move |request| {
        router!(request,
            (GET) (/) => {
                serve_html("html/index.html")
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
