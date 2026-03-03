import { describe, expect, it } from "vitest";
import { repeat } from "./vrm";

describe("vrm repeat helper", () => {
  it("creates forever repeat", () => {
    expect(repeat.forever()).toEqual({ type: "forever" });
  });

  it("creates never repeat", () => {
    expect(repeat.never()).toEqual({ type: "never" });
  });

  it("creates count repeat for a positive integer", () => {
    expect(repeat.count(3)).toEqual({ type: "count", count: 3 });
  });

  it("throws RangeError for invalid count", () => {
    expect(() => repeat.count(0)).toThrow(RangeError);
    expect(() => repeat.count(-1)).toThrow(RangeError);
    expect(() => repeat.count(1.5)).toThrow(RangeError);
    expect(() => repeat.count(Number.NaN)).toThrow(RangeError);
  });
});
