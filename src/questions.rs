use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

struct Query;

#[Object]
impl Query {
	/// Returns the sum of a and b
	async fn add(&self, a: i32, b: i32) -> i32 {
		a + b
	}
}

/// takes in a request body and resolves all the fields in the GraphQL query
pub async fn graphql_handler(req: GraphQLRequest) -> GraphQLResponse {
	let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

	schema
		.execute(req.into_inner())
		.await
		.into()
}
