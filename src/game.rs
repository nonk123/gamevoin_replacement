use rouille::websocket;
use rouille::websocket::Message;
use rouille::{Request, Response};

use std::sync::{Arc, Mutex};
use std::thread;

struct Player {
    name: String,
}

impl Player {
    fn new(name: String) -> Self {
        Self { name }
    }
}

pub struct Game {
    players: Vec<Mutex<Player>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }
}

pub fn accept_ws(game: Arc<Mutex<Game>>, request: &Request) -> Response {
    let (response, websocket) = try_or_400!(websocket::start(&request, Some("echo")));

    thread::spawn(move || {
        let mut ws = websocket.recv().unwrap();

        let player = Mutex::new(match ws.next() {
            Some(message) => match message {
                Message::Text(name) => Player::new(name),
                _ => return,
            },
            None => return,
        });

        if let Ok(_) =
            ws.send_text(format!("Good. Welcome, {}.", player.lock().unwrap().name).as_str())
        {
            game.lock().unwrap().players.push(player);
        }
    });

    response
}
