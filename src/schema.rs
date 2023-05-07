use std::fs::read_to_string;

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use serde::Deserialize;

pub struct Query;

#[Object]
impl Query {
	async fn questions<'ctx>(
		&self,
		ctx: &Context<'ctx>,
	) -> Result<&'ctx Questions, async_graphql::Error> {
		ctx.data::<Questions>()
	}
}

pub async fn build_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
	let json = get_questions().await;

	Schema::build(Query, EmptyMutation, EmptySubscription)
		.data(json)
		.finish()
}

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

/// A top level field for all questions
#[derive(SimpleObject, Deserialize)]
struct Questions {
	// needed since &str is not owned, and serde cannot deserialize borrowed data
	category: QuestionCategory,
}

// this must be a struct since GraphQL enums may only contain unit variants
/// The main categories of interview questions
/// Behavioral is more broad, regarding reactions to a situation
/// Technical is more targeted to assess knowledge in a domain
#[derive(SimpleObject, Deserialize)]
struct QuestionCategory {
	behavioral: Vec<Question>,
	technical: TechnicalQuestionSubcategory,
}

/// Broad questions
#[derive(SimpleObject, Deserialize)]
struct Question {
	html_content: String,
}

/// Domains of knowledge common in front end interviews
#[derive(SimpleObject, Deserialize)]
struct TechnicalQuestionSubcategory {
	html: Vec<Question>,
	css: Vec<Question>,
	java_script: Vec<Question>,
	node_js: Vec<Question>,
	cs_theory: Vec<Question>,
}
