use std::fs::read_to_string;

use async_graphql::{Object, SimpleObject};
use serde::Deserialize;

pub struct Query;

#[Object]
impl Query {
	async fn questions(&self) -> Questions {
		Questions {
			category: QuestionCategory {
				behavioral: vec![Question {
					html_content: "hi".to_owned(),
				}],
				technical: TechnicalQuestionSubcategory {
					html: vec![Question {
						html_content: "hello".to_owned(),
					}],
					css: vec![],
					java_script: vec![Question {
						html_content: "Beans".to_owned(),
					}],
					node_js: vec![],
					cs_theory: vec![],
				},
			},
		}
		// serde_json::from_str(&get_questions_json())
	}
}

fn get_questions_json() -> String {
	const PATH: &str = "./question_bank/questionBank.json";

	match read_to_string(PATH) {
		Ok(file) => file,
		Err(error) => panic!("Failed to open file at {PATH}: {error}"),
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
