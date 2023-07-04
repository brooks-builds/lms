import { Page } from "@playwright/test";
import { learnerInfoMockData, userinfoMockData } from "./mock_data";
import * as dotenv from "dotenv";

dotenv.config();

const AUTH0_DOMAIN = process.env.AUTH0_DOMAIN;

export enum Role {
  Learner = "Learner",
  Author = "Author",
  None = "",
  Public = "public"
}

const userInfo = {
  [Role.Learner]: learnerInfoMockData(),
  [Role.Author]: userinfoMockData(),
};

export async function login(
  role: Role,
  page: Page,
  destination: string = "/"
): Promise<void> {
  await page.goto("/", { waitUntil: "networkidle" });
  if (role != Role.None) {
    await page.route(`${AUTH0_DOMAIN}/userinfo`, async (route) => {
      if (0 == role.length) {
        return;
      }

      await route.fulfill({ json: userInfo[role] });
    });
    await page.context().addCookies([
      {
        name: "auth_token",
        value: "1234qwfp1234qwfp",
        url: page.url(),
      },
    ]);
  }

  await page.goto(destination, { waitUntil: "networkidle" });
}
