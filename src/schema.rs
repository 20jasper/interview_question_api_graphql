use async_graphql::{Enum, Object, SimpleObject};

pub struct Query;

#[Object]
impl Query {
	async fn questions(&self) -> Questions {
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

/// A top level
#[derive(SimpleObject)]
struct Questions<'a> {
	question_category: QuestionCategory<'a>,
}

// this must be a struct since GraphQL enums may only contain unit variants
#[derive(SimpleObject)]
struct QuestionCategory<'a> {
	behavioral: Vec<BehavioralQuestion<'a>>,
	technical: Vec<TechnicalQuestion<'a>>,
}

#[derive(SimpleObject)]
struct BehavioralQuestion<'a> {
	html_content: &'a str,
}

#[derive(SimpleObject)]
struct TechnicalQuestion<'a> {
	html_content: &'a str,
	subcategory: TechnicalQuestionSubtype,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum TechnicalQuestionSubtype {
	HTML,
	CSS,
	JavaScript,
	NodeJs,
	CSTheory,
}
