import { Page } from "@playwright/test";
import { courseListMockData } from "../mock_data";
import lmsArticlesMockData from "./get_lms_article_titles.json";
import lmsCourseByPk from "./lms_courses_by_pk.json";
import setLmsCourseArticlesData from "./set_lms_course_articles.json";

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
  SetLmsCourseArticles: () => setLmsCourseArticlesData,
};
