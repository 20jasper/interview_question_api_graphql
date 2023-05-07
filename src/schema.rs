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
