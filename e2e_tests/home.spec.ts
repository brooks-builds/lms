import { expect, test } from "@playwright/test";

test("has title", async ({ page }) => {
	await page.goto("/");
	await expect(page).toHaveTitle(/Brooks Builds/)
});

test("can join discord", async ({ page }) => {
	await page.goto("/");
	const discordLink = page.getByRole("link", { name: "Join Discord" });
	await expect(discordLink).toHaveAttribute("href", /discord.gg/)
});
