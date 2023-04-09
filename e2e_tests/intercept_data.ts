interface CreateAccountGraphQlStatement {
	variables: CreateAccountGraphQlUser;
	query: string;
	operationName: string;
}

interface CreateAccountGraphQlUser {
	email: string;
	password: string;
}

export function createAccount(email: string, password: string): CreateAccountGraphQlStatement {
	return {
		"variables": {
			"email": `${email}`,
			"password": `${password}`
		},
		"query": "mutation CreateLmsAccount($email: String!, $password: String!) {\n  create_account(user: {email: $email, password: $password}) {\n    _id\n  }\n}\n",
		"operationName": "CreateLmsAccount"
	}
}

export function createArticleIntercept(content: string, title: string) {
	return {"variables":{content,title},"query":"mutation InsertLmsArticle($content: String!, $title: String!) {\n  insert_lms_articles_one(object: {content: $content, title: $title}) {\n    id\n  }\n}\n","operationName":"InsertLmsArticle"}
}
