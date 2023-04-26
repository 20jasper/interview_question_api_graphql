use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

struct Query;

#[Object]
impl Query {
	/// Returns the sum of a and b
	async fn add(&self, a: i32, b: i32) -> i32 {
		a + b
	}
}

pub async fn graphql_handler() -> String {
	let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
	let res = schema
		.execute("{ add(a: 10, b: 20) }")
		.await;

	let json = serde_json::to_string(&res);
	match json {
		Ok(json) => json,
		Err(error) => panic!("failed to parse JSON: {error}"),
	}
}
