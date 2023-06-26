import { Page } from "@playwright/test";
import apiGetAllData from "./api_get_all_data.json";

const GRAPHQL_URI =
  process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

export async function interceptGraphql(page: Page): Promise<void> {
  await page.route(GRAPHQL_URI, async (route) => {
    const { operationName } = route.request().postDataJSON();

    route.fulfill({ json: { data: mockData[operationName] } });
  });
}

const mockData = {
  ApiGetAllData: apiGetAllData,
};
