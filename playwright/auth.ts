import { Page, expect } from "@playwright/test";

export async function login(page: Page, role: UserRole ) {
  const url = process.env.TEST_URL || 'http://localhost:8080'
  const user = users[role]

  await page.goto(url);
  await page.getByRole('link', { name: 'Login or Signup' }).click();
  await page.getByLabel('Email address*').fill(user.username);
  await page.getByLabel('Password*').click();
  await page.getByLabel('Password*').fill(user.password);
  await page.getByRole('button', { name: 'Continue', exact: true }).click();
  await expect(page.getByText(user.roleName)).toBeVisible();
}

export enum UserRole {
  author = 'author',
  learner = 'learner',
}

const users = {
  [UserRole.author]: {
    username: process.env.TEST_AUTHOR_USERNAME || '',
    password: process.env.TEST_AUTHOR_PASSWORD || '',
    roleName: 'Author',
    },
  [UserRole.learner]: {
    username: process.env.TEST_LEARNER_USERNAME || '',
    password: process.env.TEST_LEARNER_PASSWORD || '',
    roleName: 'Learner',
    },
}

