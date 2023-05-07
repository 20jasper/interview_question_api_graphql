use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use crate::schema::build_schema;

/// takes in a request body and resolves all the fields in the GraphQL query
pub async fn graphql_handler(req: GraphQLRequest) -> GraphQLResponse {
	let schema = build_schema().await;

	schema
		.execute(req.into_inner())
		.await
		.into()
}
