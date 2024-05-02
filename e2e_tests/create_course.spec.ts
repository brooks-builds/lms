import test, { expect } from "@playwright/test";
import {login, UserRole} from '../playwright/auth'

test('visitors cannot visit create courses', async ({ page }) => {
  await page.goto('http://localhost:8080/');
  await expect(page.getByRole('link', { name: 'Login or Signup' })).toBeVisible();
  await page.goto('http://localhost:8080/create_course');
  await expect(page.getByText('Only Authors can create a course')).toBeVisible();
  expect(page.url()).not.toContain("create_course");
});

test('learners cannot visit create courses', async ({ page }) => {
  await login(page, UserRole.learner);
  await page.goto('http://localhost:8080/create_course');
  await expect(page.getByText('Only Authors can create a course')).toBeVisible();
  expect(page.url()).not.toContain("create_course");
});
