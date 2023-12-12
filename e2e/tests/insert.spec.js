// @ts-check
const { test, expect } = require("@playwright/test");

test.beforeEach(async ({ page }) => {
  await page.goto("http://0.0.0.0:3000/");
  // delete existing measurements
  await page.getByLabel("delete").click();
});

test("Insert measurement", async ({ page }) => {
  // insert a measurement
  await page.getByLabel("insert").click();

  // ensure inserting worked
  expect(await page.getByLabel("measurement").count()).toEqual(1);
});
