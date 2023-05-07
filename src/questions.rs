use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use std::fs::read_to_string;

use crate::schema::{Query, Questions};

/// Import JSON from the question bank file then convert to a rust `Questions` struct
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

/// takes in a request body and resolves all the fields in the GraphQL query
/// Has access to all interview questions in its context
pub async fn graphql_handler(req: GraphQLRequest) -> GraphQLResponse {
	let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
		.data(get_questions().await)
		.finish();

	schema
		.execute(req.into_inner())
		.await
		.into()
}
