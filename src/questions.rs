use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use crate::schema::Query;

/// takes in a request body and resolves all the fields in the GraphQL query
pub async fn graphql_handler(req: GraphQLRequest) -> GraphQLResponse {
	let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

	schema
		.execute(req.into_inner())
		.await
		.into()
}
