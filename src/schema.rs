use std::fs::read_to_string;

use async_graphql::{Enum, Object, SimpleObject};
use serde::{Deserialize, Serialize};

pub struct Query;

#[Object]
impl Query {
	async fn questions(&self) -> Questions {
		match serde_json::from_str(&get_questions_json()) {
			Ok(json) => json,
			Err(error) => panic!("Failed to parse json to type Questions: {error}"),
		}
	}
}

fn get_questions_json() -> String {
	const PATH: &str = "../question_bank/questionBank.json";

	match read_to_string(PATH) {
		Ok(file) => file,
		Err(error) => panic!("Failed to open file at {PATH}: {error}"),
	}
}

/// A top level field for all questions
#[derive(SimpleObject, Deserialize, Serialize)]
struct Questions {
	// needed since &str is not owned, and serde cannot deserialize borrowed data
	category: QuestionCategory,
}

// this must be a struct since GraphQL enums may only contain unit variants
/// The main categories of interview questions—behavioral being more broad and technical more targeted
#[derive(SimpleObject, Serialize, Deserialize)]
struct QuestionCategory {
	behavioral: Vec<BehavioralQuestion>,
	technical: Vec<TechnicalQuestion>,
}

/// Broad questions regarding reactions to a situation
#[derive(SimpleObject, Serialize, Deserialize)]
struct BehavioralQuestion {
	html_content: String,
}

/// Targeted questions to assess knowledge in a domain
#[derive(SimpleObject, Serialize, Deserialize)]
struct TechnicalQuestion {
	html_content: String,
	subcategory: TechnicalQuestionSubcategory,
}

/// Domains of knowledge common in front end interviews
#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
enum TechnicalQuestionSubcategory {
	Html,
	Css,
	JavaScript,
	NodeJs,
	CSTheory,
}
