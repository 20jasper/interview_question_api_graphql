use axum::{
	routing::{get, post},
	Router,
};
use std::env;
use std::net::SocketAddr;

mod questions;
mod schema;

#[tokio::main]
async fn main() {
	let port = get_port();

	// set application routes
	let app = Router::new()
		.route("/", get(hello_world))
		.route("/api/graphql", post(questions::graphql_handler));

	println!("Go to http://localhost:{port}/ to see your amazing app");
	// run app with hyper
	let address = SocketAddr::from(([0, 0, 0, 0], port));
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
			default_port
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::get_port;
	use std::env;

	#[test]
	fn should_be_3000_if_there_is_no_environment_variable() {
		env::remove_var("PORT");

		let port = get_port();

		assert_eq!(port, 3000)
	}
	#[test]
	fn should_be_the_value_set_in_the_environment_variable() {
		env::set_var("PORT", "2121");

		let port = get_port();

		assert_eq!(port, 2121);

		env::remove_var("PORT");
	}
}
