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

impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}



fn main() {
    server::new(|| App::new()
        .resource("/", |r| r.f(index))
        .resource("/ws/", |r| r.method(http::Method::GET).f(ws_index))
    )
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
