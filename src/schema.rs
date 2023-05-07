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

/// A top level field for all questions
#[derive(SimpleObject)]
struct Questions<'a> {
	category: QuestionCategory<'a>,
}

// this must be a struct since GraphQL enums may only contain unit variants
/// The main categories of interview questions—behavioral being more broad and technical more targeted
#[derive(SimpleObject)]
struct QuestionCategory<'a> {
	behavioral: Vec<BehavioralQuestion<'a>>,
	technical: Vec<TechnicalQuestion<'a>>,
}

/// Broad questions regarding reactions to a situation
#[derive(SimpleObject)]
struct BehavioralQuestion<'a> {
	html_content: &'a str,
}

/// Targeted questions to assess knowledge in a domain
#[derive(SimpleObject)]
struct TechnicalQuestion<'a> {
	html_content: &'a str,
	subcategory: TechnicalQuestionSubcategory,
}

/// Domains of knowledge common in front end interviews
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum TechnicalQuestionSubcategory {
	HTML,
	CSS,
	JavaScript,
	NodeJs,
	CSTheory,
}
