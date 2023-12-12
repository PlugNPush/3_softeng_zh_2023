// @ts-check
const { test, expect } = require('@playwright/test');

test('has title', async ({ page }) => {
  await page.goto('http://0.0.0.0:3000/');

  // Expect a title "to contain" a substring.
  await expect(page).toHaveTitle(/Temperature Dashboard/);
});
