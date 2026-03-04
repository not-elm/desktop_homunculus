const DEFAULT_TIMEOUT_MS = 30_000;

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
