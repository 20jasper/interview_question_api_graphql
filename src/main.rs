use axum::{routing::get, Router};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
	let port = get_port();

	// set application routes
	let app = Router::new().route("/", get(hello_world));

	// run app with hyper
	let address = SocketAddr::from(([127, 0, 0, 1], port));
	axum::Server::bind(&address)
		.serve(app.into_make_service())
		.await
		.expect("Server failed to start at address {address}");
}

async fn hello_world() -> String {
	"hello world".to_owned()
}

fn get_port() -> u16 {
	match env::var("PORT") {
		Ok(unparsed_port) => match unparsed_port.parse() {
			Ok(parsed_port) => parsed_port,
			Err(error) => panic!("could not parse int {unparsed_port}: {error}"),
		},
		Err(error) => {
			let default_port: u16 = 3000;
			println!("$PORT is not set, using default port {default_port}: {error}");
			println!("Go to http://localhost:{default_port}/ to see your amazing app");
			default_port
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::get_port;

	#[test]
	fn should_be_3000_if_there_is_no_environment_variable() {
		let port = get_port();

		assert_eq!(port, 3000)
	}
}
