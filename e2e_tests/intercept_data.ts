interface CreateAccountGraphQlStatement {
  variables: CreateAccountGraphQlUser;
  query: string;
  operationName: string;
}

interface CreateAccountGraphQlUser {
  email: string;
  password: string;
}

export function createAccount(
  email: string,
  password: string
): CreateAccountGraphQlStatement {
  return {
    variables: {
      email: `${email}`,
      password: `${password}`,
    },
    query:
      "mutation CreateLmsAccount($email: String!, $password: String!) {\n  create_account(user: {email: $email, password: $password}) {\n    _id\n  }\n}\n",
    operationName: "CreateLmsAccount",
  };
}

export function createArticleIntercept(content: string, title: string) {
  return {
    variables: { content, title },
    query:
      "mutation InsertLmsArticle($content: String!, $title: String!) {\n  insert_lms_articles_one(object: {content: $content, title: $title}) {\n    id\n  }\n}\n",
    operationName: "InsertLmsArticle",
  };
}

export function addArticleToCourseIntercept(id: number, article_ids: number[]) {
  return {
    variables: { id, article_ids },
    query: `mutation SetLmsCourseArticles($id: Int, $article_ids: json) {
      update_lms_courses_by_pk(pk_columns: {id: $id}, _set: {article_ids: $article_ids}) {
        id
        article_ids
      }
    }
    `,
    operationName: "SetLmsCourseArticles",
  };
}
