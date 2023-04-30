use async_graphql::{EmptyMutation, EmptySubscription, Enum, Object, Schema, SimpleObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

struct Query;

#[Object]
impl Query {
	async fn questions(&self) -> Vec<Question> {
		vec![
			Question {
				html_content: "what's good".to_owned(),
				question_category: QuestionCategory::Behavioral,
			},
			Question {
				html_content: "what's bad".to_owned(),
				question_category: QuestionCategory::Technical,
			},
		]
	}
}

#[derive(SimpleObject)]
struct Question {
	html_content: String,
	question_category: QuestionCategory,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum QuestionCategory {
	Behavioral,
	Technical,
}

/// takes in a request body and resolves all the fields in the GraphQL query
pub async fn graphql_handler(req: GraphQLRequest) -> GraphQLResponse {
	let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

	schema
		.execute(req.into_inner())
		.await
		.into()
}
