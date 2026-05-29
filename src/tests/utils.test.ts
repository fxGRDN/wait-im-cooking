import { describe, it, expect } from "vitest";
import { COMMON_UNITS } from "../lib/utils";

describe("utils", () => {
  it("should have the expected common units", () => {
    expect(COMMON_UNITS).toEqual(["g", "kg", "ml", "l", "pcs"]);
  });
});
