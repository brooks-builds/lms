use graphql_client::GraphQLQuery;
use ycl::foundations::roles::BBRole;

use crate::api::SendToGraphql;
use crate::database_queries::{
    api_get_article_titles_by_ids, get_lms_article_titles, insert_lms_article,
    ApiGetArticleTitlesByIds, GetLmsArticleTitles, InsertLmsArticle,
};
use crate::errors::LmsError;
use crate::stores::articles::{Article, ArticlesStore};

pub async fn create_article(
    title: String,
    content: String,
    token: String,
) -> Result<insert_lms_article::ResponseData, LmsError> {
    let variables = insert_lms_article::Variables { title, content };
    let query = InsertLmsArticle::build_query(variables);
    SendToGraphql::new()
        .json(query)?
        .authorization(&token)
        .role(BBRole::Author)
        .send()
        .await
}

pub async fn get_article_titles(token: String) -> Result<ArticlesStore, LmsError> {
    let variables = get_lms_article_titles::Variables {};
    let query = GetLmsArticleTitles::build_query(variables);
    let response = SendToGraphql::new()
        .authorization(&token)
        .role(BBRole::Author)
        .json(query)?
        .send::<get_lms_article_titles::ResponseData>()
        .await?;

    Ok(response.into())
}

pub async fn get_article_titles_by_ids(article_ids: Vec<i64>) -> Result<Vec<Article>, LmsError> {
    let variables = api_get_article_titles_by_ids::Variables { article_ids };
    let query = ApiGetArticleTitlesByIds::build_query(variables);
    let response = SendToGraphql::new()
        .json(query)?
        .send::<api_get_article_titles_by_ids::ResponseData>()
        .await?;
    let articles = response
        .lms_articles
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Article>>();

    Ok(articles)
}
