/**
 * Resolves after `ms` milliseconds (non-blocking delay).
 *
 * @example
 *
 */
export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
