const DEFAULT_TIMEOUT_MS = 30_000;

/**
 * Write a structured error to stderr and exit with the given code.
 */
export function fail(code: string, message: string, exitCode: number): never {
  console.error(JSON.stringify({ code, message }));
  process.exit(exitCode);
}

/**
 * Fetch with a timeout. Rejects if the request takes longer than `ms`.
 */
export async function fetchWithTimeout(
  url: string,
  options: RequestInit,
  ms: number = DEFAULT_TIMEOUT_MS,
): Promise<Response> {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), ms);
  try {
    return await fetch(url, { ...options, signal: controller.signal });
  } finally {
    clearTimeout(timer);
  }
}
