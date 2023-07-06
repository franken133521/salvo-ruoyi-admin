mod router;
mod controller;
mod model;

use once_cell::sync::Lazy;
use redis::Client;
use salvo::{prelude::TcpListener, Server, Listener, handler};

#[handler]
async fn hello() -> &'static str {
    "Hello World!"
}

pub static GLOBAL_REDIS: Lazy<Client> = Lazy::new(|| {
    let client = redis::Client::open("redis://127.0.0.1/").expect("redis连接失败");
    client.get_connection().expect("redis连接失败");
    client
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = router::init();
    let acceptor = TcpListener::new("0.0.0.0:8090").bind().await;
    Server::new(acceptor).serve(router).await;
}