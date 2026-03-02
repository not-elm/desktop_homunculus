import { describe, it, expect, vi, beforeEach } from "vitest";
import { z } from "zod";

describe("input.parse", () => {
  // eslint-disable-next-line @typescript-eslint/consistent-type-imports
  let input: typeof import("./commands").input;
  // eslint-disable-next-line @typescript-eslint/consistent-type-imports
  let StdinParseError: typeof import("./commands").StdinParseError;

  beforeEach(async () => {
    vi.resetModules();
  });

  async function loadWithMockedStdin(data: string) {
    const encoder = new TextEncoder();
    const chunks = [Buffer.from(encoder.encode(data))];
    const mockStdin = {
      [Symbol.asyncIterator]: async function* () {
        for (const chunk of chunks) {
          yield chunk;
        }
      },
    };
    vi.spyOn(process, "stdin", "get").mockReturnValue(mockStdin as unknown as typeof process.stdin);

    const mod = await import("./commands");
    input = mod.input;
    StdinParseError = mod.StdinParseError;
  }

  it("parses valid JSON matching schema", async () => {
    await loadWithMockedStdin('{"name":"alice","age":30}');
    const schema = z.object({ name: z.string(), age: z.number() });
    const result = await input.parse(schema);
    expect(result).toEqual({ name: "alice", age: 30 });
  });

  it("applies zod defaults for missing optional fields", async () => {
    await loadWithMockedStdin("{}");
    const schema = z.object({
      speaker: z.number().default(0),
      host: z.string().default("http://localhost:50021"),
    });
    const result = await input.parse(schema);
    expect(result).toEqual({ speaker: 0, host: "http://localhost:50021" });
  });

  it("throws EMPTY_STDIN when stdin is empty", async () => {
    await loadWithMockedStdin("");
    const schema = z.object({});
    await expect(input.parse(schema)).rejects.toThrow(StdinParseError);
    await loadWithMockedStdin("");
    try {
      await input.parse(schema);
    } catch (err) {
      expect((err as InstanceType<typeof StdinParseError>).code).toBe("EMPTY_STDIN");
    }
  });

  it("throws EMPTY_STDIN when stdin is whitespace-only", async () => {
    await loadWithMockedStdin("   \n  ");
    const schema = z.object({});
    await expect(input.parse(schema)).rejects.toThrow(StdinParseError);
  });

  it("throws INVALID_JSON when stdin is not valid JSON", async () => {
    await loadWithMockedStdin("not json {{{");
    const schema = z.object({});
    try {
      await input.parse(schema);
      expect.unreachable("should have thrown");
    } catch (err) {
      expect(err).toBeInstanceOf(StdinParseError);
      expect((err as InstanceType<typeof StdinParseError>).code).toBe("INVALID_JSON");
    }
  });

  it("throws VALIDATION_ERROR when JSON does not match schema", async () => {
    await loadWithMockedStdin('{"name":123}');
    const schema = z.object({ name: z.string() });
    try {
      await input.parse(schema);
      expect.unreachable("should have thrown");
    } catch (err) {
      expect(err).toBeInstanceOf(StdinParseError);
      const e = err as InstanceType<typeof StdinParseError>;
      expect(e.code).toBe("VALIDATION_ERROR");
      expect(e.details).toBeDefined();
    }
  });
});

describe("input.parseMenu", () => {
  // eslint-disable-next-line @typescript-eslint/consistent-type-imports
  let input: typeof import("./commands").input;
  // eslint-disable-next-line @typescript-eslint/consistent-type-imports
  let StdinParseError: typeof import("./commands").StdinParseError;

  beforeEach(async () => {
    vi.resetModules();
  });

  async function loadWithMockedStdin(data: string) {
    const encoder = new TextEncoder();
    const chunks = [Buffer.from(encoder.encode(data))];
    const mockStdin = {
      [Symbol.asyncIterator]: async function* () {
        for (const chunk of chunks) {
          yield chunk;
        }
      },
    };
    vi.spyOn(process, "stdin", "get").mockReturnValue(mockStdin as unknown as typeof process.stdin);

    const mod = await import("./commands");
    input = mod.input;
    StdinParseError = mod.StdinParseError;
  }

  it("returns a Vrm with the correct entity", async () => {
    await loadWithMockedStdin('{"linkedVrm":42}');
    const vrm = await input.parseMenu();
    expect(vrm.entity).toBe(42);
  });

  it("returns an instance of Vrm", async () => {
    await loadWithMockedStdin('{"linkedVrm":1}');
    const { Vrm } = await import("./vrm");
    const vrm = await input.parseMenu();
    expect(vrm).toBeInstanceOf(Vrm);
  });

  it("throws EMPTY_STDIN when stdin is empty", async () => {
    await loadWithMockedStdin("");
    try {
      await input.parseMenu();
      expect.unreachable("should have thrown");
    } catch (err) {
      expect(err).toBeInstanceOf(StdinParseError);
      expect((err as InstanceType<typeof StdinParseError>).code).toBe("EMPTY_STDIN");
    }
  });

  it("throws INVALID_JSON when stdin is not valid JSON", async () => {
    await loadWithMockedStdin("not json");
    try {
      await input.parseMenu();
      expect.unreachable("should have thrown");
    } catch (err) {
      expect(err).toBeInstanceOf(StdinParseError);
      expect((err as InstanceType<typeof StdinParseError>).code).toBe("INVALID_JSON");
    }
  });

  it("throws VALIDATION_ERROR when linkedVrm is wrong type", async () => {
    await loadWithMockedStdin('{"linkedVrm":"not-a-number"}');
    try {
      await input.parseMenu();
      expect.unreachable("should have thrown");
    } catch (err) {
      expect(err).toBeInstanceOf(StdinParseError);
      expect((err as InstanceType<typeof StdinParseError>).code).toBe("VALIDATION_ERROR");
    }
  });

  it("throws VALIDATION_ERROR when linkedVrm is missing", async () => {
    await loadWithMockedStdin('{"other":123}');
    try {
      await input.parseMenu();
      expect.unreachable("should have thrown");
    } catch (err) {
      expect(err).toBeInstanceOf(StdinParseError);
      expect((err as InstanceType<typeof StdinParseError>).code).toBe("VALIDATION_ERROR");
    }
  });
});

describe("input.read", () => {
  it("reads all chunks as a utf-8 string", async () => {
    const chunks = [Buffer.from("hello "), Buffer.from("world")];
    const mockStdin = {
      [Symbol.asyncIterator]: async function* () {
        for (const chunk of chunks) {
          yield chunk;
        }
      },
    };
    vi.spyOn(process, "stdin", "get").mockReturnValue(mockStdin as unknown as typeof process.stdin);

    const { input } = await import("./commands");
    const result = await input.read();
    expect(result).toBe("hello world");
  });
});

describe("output.write", () => {
  beforeEach(() => {
    vi.resetModules();
    vi.restoreAllMocks();
  });

  it("writes JSON to fd 1", async () => {
    vi.doMock("node:fs", () => ({
      writeFileSync: vi.fn(),
    }));
    const { output } = await import("./commands");
    const fs = await import("node:fs");
    output.write({ ok: true });
    expect(fs.writeFileSync).toHaveBeenCalledWith(1, '{"ok":true}\n');
  });

  it("writes null as JSON", async () => {
    vi.doMock("node:fs", () => ({
      writeFileSync: vi.fn(),
    }));
    const { output } = await import("./commands");
    const fs = await import("node:fs");
    output.write(null);
    expect(fs.writeFileSync).toHaveBeenCalledWith(1, "null\n");
  });

  it("falls back to error JSON when data is not serializable", async () => {
    vi.doMock("node:fs", () => ({
      writeFileSync: vi.fn(),
    }));
    const { output } = await import("./commands");
    const fs = await import("node:fs");
    const circular: Record<string, unknown> = {};
    circular.self = circular;
    output.write(circular);
    expect(fs.writeFileSync).toHaveBeenCalledWith(
      1,
      '{"code":"SERIALIZE_ERROR","message":"Failed to serialize output"}\n',
    );
  });
});

describe("output.writeError", () => {
  beforeEach(() => {
    vi.resetModules();
    vi.restoreAllMocks();
  });

  it("writes structured error to fd 2", async () => {
    vi.doMock("node:fs", () => ({
      writeFileSync: vi.fn(),
    }));
    const { output } = await import("./commands");
    const fs = await import("node:fs");
    output.writeError("NOT_FOUND", "entity missing");
    expect(fs.writeFileSync).toHaveBeenCalledWith(
      2,
      '{"code":"NOT_FOUND","message":"entity missing"}\n',
    );
  });
});

describe("output.succeed", () => {
  beforeEach(() => {
    vi.resetModules();
    vi.restoreAllMocks();
  });

  it("writes result to fd 1 and exits with code 0", async () => {
    vi.doMock("node:fs", () => ({
      writeFileSync: vi.fn(),
    }));
    const exitSpy = vi.spyOn(process, "exit").mockImplementation(() => undefined as never);
    const { output } = await import("./commands");
    const fs = await import("node:fs");
    output.succeed({ ok: true });
    expect(fs.writeFileSync).toHaveBeenCalledWith(1, '{"ok":true}\n');
    expect(exitSpy).toHaveBeenCalledWith(0);
  });
});

describe("output.fail", () => {
  beforeEach(() => {
    vi.resetModules();
    vi.restoreAllMocks();
  });

  it("writes error to fd 2 and exits with given code", async () => {
    vi.doMock("node:fs", () => ({
      writeFileSync: vi.fn(),
    }));
    const exitSpy = vi.spyOn(process, "exit").mockImplementation(() => undefined as never);
    const { output } = await import("./commands");
    const fs = await import("node:fs");
    output.fail("BAD_INPUT", "missing name", 2);
    expect(fs.writeFileSync).toHaveBeenCalledWith(
      2,
      '{"code":"BAD_INPUT","message":"missing name"}\n',
    );
    expect(exitSpy).toHaveBeenCalledWith(2);
  });

  it("defaults exitCode to 1", async () => {
    vi.doMock("node:fs", () => ({
      writeFileSync: vi.fn(),
    }));
    const exitSpy = vi.spyOn(process, "exit").mockImplementation(() => undefined as never);
    const { output } = await import("./commands");
    output.fail("ERR", "something broke");
    expect(exitSpy).toHaveBeenCalledWith(1);
  });
});

describe("StdinParseError", () => {
  it("is an instance of Error with correct properties", async () => {
    const { StdinParseError } = await import("./commands");
    const err = new StdinParseError("EMPTY_STDIN", "no input");
    expect(err).toBeInstanceOf(Error);
    expect(err.name).toBe("StdinParseError");
    expect(err.code).toBe("EMPTY_STDIN");
    expect(err.message).toBe("no input");
    expect(err.details).toBeUndefined();
  });

  it("includes details when provided", async () => {
    const { StdinParseError } = await import("./commands");
    const detail = { issues: ["bad field"] };
    const err = new StdinParseError("VALIDATION_ERROR", "failed", detail);
    expect(err.details).toBe(detail);
  });
});
