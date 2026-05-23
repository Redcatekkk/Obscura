import { describe, expect, it } from "vitest";
import { categories, deckFunctions, enabledModuleCount, moduleIds, moduleSeedCount } from "./functions";

describe("deck function seed data", () => {
  it("contains the 20 visible mock modules", () => {
    expect(moduleSeedCount).toBe(20);
    expect(moduleIds()).toContain("f01");
    expect(moduleIds()).toContain("f20");
  });

  it("covers all five categories", () => {
    expect(categories).toEqual(["Automation", "Utility", "Privacy", "Stats", "Fun"]);
    expect(new Set(deckFunctions.map((module) => module.category)).size).toBe(5);
    expect(enabledModuleCount()).toBeGreaterThan(0);
  });
});
