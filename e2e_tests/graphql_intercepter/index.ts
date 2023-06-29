import { Page } from "@playwright/test";
import apiGetAllData from "./api_get_all_data.json";
import apiGetAllDataVisitor from "./api_get_all_data_visitor.json";
import { Role } from "../utils";

const GRAPHQL_URI =
  process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

export async function interceptGraphql(page: Page): Promise<void> {
  await page.route(GRAPHQL_URI, async (route) => {
    const { operationName } = route.request().postDataJSON();
    const role = await route.request().headerValue("x-hasura-role") || Role.Public;

    route.fulfill({ json: { data: mockDataByRole[role][operationName] } });
  });
}

const mockData = {
  ApiGetAllData: apiGetAllData,
};

const mockDataVisitor = {
  ApiGetAllData: apiGetAllDataVisitor,
}

const mockDataByRole = {
  public: mockDataVisitor,
  Learner: mockData,
  Author: mockData
}
