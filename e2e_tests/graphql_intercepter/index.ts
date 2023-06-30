import { Page } from "@playwright/test";
import apiGetAllData from "./api_get_all_data.json";
import apiGetAllDataVisitor from "./api_get_all_data_visitor.json";
import apiInsertUserArticle from "./api_insert_user_articles.json";
import { Role } from "../utils";
import apiCompleteUserArticle from "./api_complete_user_article.json";
import apiInsertCourseArticles from "./api_insert_course_articles.json";

const GRAPHQL_URI =
  process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

export async function interceptGraphql(page: Page): Promise<void> {
  await page.route(GRAPHQL_URI, async (route) => {
    const { operationName } = route.request().postDataJSON();
    const role = await route.request().headerValue("x-hasura-role") || Role.Public;
    console.log(role, operationName);

    route.fulfill({ json: { data: mockDataByRole[role][operationName] } });
  });
}

const mockData = {
  ApiGetAllData: apiGetAllData,
  ApiInsertUserArticle: apiInsertUserArticle,
  ApiCompleteUserArticle: apiCompleteUserArticle,
  ApiInsertCourseArticles: apiInsertCourseArticles
};

const mockDataVisitor = {
  ApiGetAllData: apiGetAllDataVisitor,
}

const mockDataByRole = {
  public: mockDataVisitor,
  Learner: mockData,
  Author: mockData
}
