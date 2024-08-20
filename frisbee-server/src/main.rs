use std::sync::mpsc;

use salvo::prelude::*;
use serde::Deserialize;
use salvo::logging::Logger;
use log::info;

mod fetcher;

#[derive(Debug, Default, Deserialize)]
struct Payload {
    url: String,
}

#[handler]
async fn receiver(req: &mut Request, depot: &mut Depot) -> String {
    let payload = req.parse_json::<Payload>().await.unwrap();
    let output = format!("got: {:?}", payload);
    info!("{}", output);
    let tx = depot.obtain::<mpsc::Sender<String>>().unwrap();
    tx.send(payload.url).unwrap();
    output
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (tx, handle) = fetcher::run();
    // Now I need to somehow pass the tx to the receiver handler.

    let router = Router::new().post(receiver);
    let service = Service::new(router)
        .hoop(affix_state::inject(tx))
        .hoop(Logger::new());
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(service).await;
}
