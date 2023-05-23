import { Page } from "@playwright/test";
import { courseListMockData } from "../mock_data";
import lmsArticlesMockData from "./get_lms_article_titles.json";
import lmsCourseByPk from "./lms_courses_by_pk.json";
import apiGetArticleTitlesByIds from "./api_get_article_title_by_ids.json";
import insertLmsArticle from "./insert_lms_article.json";

const GRAPHQL_URI =
  process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

export async function interceptGraphql(page: Page): Promise<void> {
  await page.route(GRAPHQL_URI, async (route) => {
    const { operationName } = route.request().postDataJSON();

    route.fulfill({ json: { data: mockData[operationName]() } });
  });
}

const mockData = {
  ListLmsCourses: () => courseListMockData,
  GetLmsArticleTitles: () => lmsArticlesMockData,
  CourseById: () => lmsCourseByPk,
  ApiGetArticleTitlesByIds: () => apiGetArticleTitlesByIds,
  InsertLmsArticle: () => insertLmsArticle
};
