import { faker } from "@faker-js/faker";

export const courseListMockData = {
  lms_courses: [
    {
      id: 1,
      lms_tag: { id: 1, name: "Yew" },
      short_description:
        "Learn how to build websites using Yew.rs, a frontend framework modeled after React.",
      title: "Yew.rs",
      long_description: "Yew.rs",
      trailer_uri: null,
      article_ids: [2],
    },
    {
      id: 2,
      lms_tag: { id: 2, name: "Axum" },
      short_description:
        "Learn how to build API's with Axum, a Rust based backend framework just like Express",
      title: "Axum",
      long_description: "Axum",
      trailer_uri: null,
      article_ids: [2],
    },
  ],
};

export const createAccountMockData = {
  data: { create_account: { _id: "64051def82a59da572bddce9" } },
};

export function userinfoMockData() {
  return {
    sub: "auth0|38947jfsuyhafiull",
    nickname: "meow",
    name: "meow@mailinator.com",
    picture:
      "https://s.gravatar.com/avatar/4aefaae80a9342bf4fd82b469021fdb5?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fme.png",
    updated_at: "2023-03-17T13:06:27.451Z",
    email: "meow@mailinator.com",
    email_verified: false,
    "https://hasura.io/jwt/claims": {
      "x-hasura-allowed-roles": ["Author", "Learner"],
      "x-hasura-default-role": "public",
      "x-hasura-user-id": "auth0|641465b9d1afd8c27810392b",
    },
    "https://brooksbuilds.com": { roles: ["Author"], role: "Author" },
  };
}

export function learnerInfoMockData() {
  return {
    sub: "auth0|38947jfsuyhafiull",
    nickname: "meow",
    name: "meow@mailinator.com",
    picture:
      "https://s.gravatar.com/avatar/4aefaae80a9342bf4fd82b469021fdb5?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fme.png",
    updated_at: "2023-03-17T13:06:27.451Z",
    email: "meow@mailinator.com",
    email_verified: false,
    "https://hasura.io/jwt/claims": {
      "x-hasura-allowed-roles": ["Learner"],
      "x-hasura-default-role": "public",
      "x-hasura-user-id": "auth0|641465b9d1afd8c27810392b",
    },
    "https://brooksbuilds.com": { roles: ["Learner"], role: "Learner" },
  };
}

export function tagsMockData() {
  const result = {
    data: {
      lms_tags: [
        {
          id: 1,
          name: "Yew",
        },
        {
          id: 2,
          name: "Axum",
        },
      ],
    },
  };

  return result;
}

export function createdTagMockData(name: string) {
  return { data: { insert_lms_tags_one: { id: 3, name: name } } };
}

export function lmsCoursByPk(
  id: number,
  title: string,
  shortDescription: string,
  longDescription: string,
  tag: string
) {
  return {
    data: {
      insert_lms_courses_one: {
        id,
        long_description: longDescription,
        short_description: shortDescription,
        lms_tag: { id: 1, name: tag },
        tag_id: 1,
        title,
        trailer_uri: null,
        price: null,
      },
    },
  };
}

export function createArticleMockData() {
  return { data: { insert_lms_articles_one: { id: 2 } } };
}

export function createArticleErrorMockData() {
  return {
    errors: [
      {
        extensions: {
          code: "validation-failed",
          path: "$.selectionSet.insert_lms_articles_one",
        },
        message:
          "field 'insert_lms_articles_one' not found in type: 'mutation_root'",
      },
    ],
  };
}

export interface LmsArticleMockData {
  id: number;
  created_at: Date;
  title: string;
}

export function lmsArticlesMockData(titles: string[]): {
  data: { lms_articles: LmsArticleMockData[] };
} {
  return {
    data: {
      lms_articles: titles.map((title, index) => {
        return {
          created_at: faker.date.past(),
          id: index,
          title,
        };
      }),
    },
  };
}

export function setCourseArticlesMockData() {
  return { data: { update_lms_courses_by_pk: { id: 1, article_ids: [1] } } };
}
