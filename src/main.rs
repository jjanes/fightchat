extern crate actix;
extern crate actix_web;
extern crate env_logger;

use actix::prelude::*;
use actix_web::{fs, http, middleware, server, ws, App, Error, HttpRequest, HttpResponse};

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn ws_index(r: HttpRequest) -> Result<HttpResponse, Error> {
    ws::start(r, MyWebSocket)
}


struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

struct ChatSession {
    id: usize,
    user_id: usize,
    sender_id: usize,
    message: String,
}

struct ChatMessage {
    id: usize,
    user_id: usize,
    sender_id: usize,
    message: String,
}




impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => {
                let m = text.trim();
                let v: Vec<&str> = m.splitn(2, ' ').collect();
                
                match v[0] { 
                    "/register" => {


                    }, 
                    "/message" => {
                    



                    },
                    _ => {

                    }

                }   
            },
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}



fn main() { 
    Command::new("ps")
        .args(&["aux", "test"])
        .output()
        .expect("failed to execute process")

    println!(output.stdout);
}

fn main_1() {
    let mut messages_array = [; 1000];
    
    server::new(|| App::new()
        .resource("/", |r| r.f(index))
        .resource("/ws/", |r| r.method(http::Method::GET).f(ws_index))
    )
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
