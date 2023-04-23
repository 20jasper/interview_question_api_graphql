use axum::{routing::get, Router};
use std::net::SocketAddr;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
	// set application routes
	let app = Router::new().route("/", get(hello_world));

	// run app with hyper
	let address = SocketAddr::from(([127, 0, 0, 1], PORT));
	axum::Server::bind(&address)
		.serve(app.into_make_service())
		.await
		.expect("Server failed to start at address {address}");
}

async fn hello_world() -> String {
	"hello world".to_owned()
}
