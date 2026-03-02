import { describe, it, expect, beforeEach } from "vitest";
import { HomunculusMcpState } from "../state.js";

describe("HomunculusMcpState", () => {
  let state: HomunculusMcpState;

  beforeEach(() => {
    state = new HomunculusMcpState();
  });

  it("should start with no active character", () => {
    expect(state.activeCharacterEntity).toBeNull();
  });

  it("should allow setting active character", () => {
    state.activeCharacterEntity = 42;
    expect(state.activeCharacterEntity).toBe(42);
  });

  it("should allow clearing active character", () => {
    state.activeCharacterEntity = 42;
    state.activeCharacterEntity = null;
    expect(state.activeCharacterEntity).toBeNull();
  });

  describe("webview tracking", () => {
    it("should start with empty webview list", () => {
      expect(state.openWebviews).toEqual([]);
    });

    it("should track opened webviews", () => {
      state.trackWebview(100);
      state.trackWebview(200);
      expect(state.openWebviews).toEqual([100, 200]);
    });

    it("should return last opened webview", () => {
      state.trackWebview(100);
      state.trackWebview(200);
      state.trackWebview(300);
      expect(state.lastWebview()).toBe(300);
    });

    it("should return null when no webviews tracked", () => {
      expect(state.lastWebview()).toBeNull();
    });

    it("should untrack a specific webview", () => {
      state.trackWebview(100);
      state.trackWebview(200);
      state.trackWebview(300);
      state.untrackWebview(200);
      expect(state.openWebviews).toEqual([100, 300]);
    });

    it("should clear all tracked webviews", () => {
      state.trackWebview(100);
      state.trackWebview(200);
      state.clearWebviews();
      expect(state.openWebviews).toEqual([]);
    });
  });
});
