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

