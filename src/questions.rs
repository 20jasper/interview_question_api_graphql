use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use std::fs::read_to_string;

use crate::schema::{Query, Questions};

async fn get_questions() -> Questions {
	const PATH: &str = "./question_bank/questionBank.json";

	let questions_json = match read_to_string(PATH) {
		Ok(file) => file,
		Err(error) => panic!("Failed to open file at {PATH}: {error}"),
	};

	match serde_json::from_str(&questions_json) {
		Ok(questions) => questions,
		Err(error) => panic!("Could not parse json to type Questions: {error}"),
	}
}

async fn build_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
	let json = get_questions().await;

	Schema::build(Query, EmptyMutation, EmptySubscription)
		.data(json)
		.finish()
}

/// takes in a request body and resolves all the fields in the GraphQL query
pub async fn graphql_handler(req: GraphQLRequest) -> GraphQLResponse {
	let schema = build_schema().await;

	schema
		.execute(req.into_inner())
		.await
		.into()
}
