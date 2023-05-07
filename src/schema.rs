use async_graphql::{Context, Object, SimpleObject};
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

/// A top level field for all questions
#[derive(SimpleObject, Deserialize)]
pub struct Questions {
	category: QuestionCategory,
}

// this must be a struct since GraphQL enums may only contain unit variants
/// The main categories of interview questions
#[derive(SimpleObject, Deserialize)]
struct QuestionCategory {
	behavioral: BehavioralQuestions,
	technical: TechnicalQuestions,
}

/// Broader questions regarding reactions to a situation
type BehavioralQuestions = Vec<Question>;

/// Questions assessing knowledge in a particular domain
#[derive(SimpleObject, Deserialize)]
struct TechnicalQuestions {
	subcategory: TechnicalQuestionSubcategory,
}

/// HTML encoded interview questions
#[derive(SimpleObject, Deserialize)]
struct Question {
	#[serde(rename(deserialize = "htmlContent"))]
	html_content: String,
}

/// Domains of knowledge common in front end interviews
#[derive(SimpleObject, Deserialize)]
struct TechnicalQuestionSubcategory {
	html: Vec<Question>,
	css: Vec<Question>,
	#[serde(rename(deserialize = "javaScript"))]
	java_script: Vec<Question>,
	#[serde(rename(deserialize = "nodeJs"))]
	node_js: Vec<Question>,
	#[serde(rename(deserialize = "csTheory"))]
	cs_theory: Vec<Question>,
}
