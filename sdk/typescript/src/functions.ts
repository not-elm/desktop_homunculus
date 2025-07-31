/**
 * Utility functions for common operations and runtime detection.
 * 
 * Provides helper functions for timing operations, environment detection,
 * and other common utilities used throughout the SDK.
 * 
 * @example
 * ```typescript
 * // Delay execution
 * await sleep(1000); // Wait 1 second
 * 
 * // Check runtime environment
 * const env = runtime();
 * if (env === 'browser') {
 *   console.log('Running in browser environment');
 * }
 * ```
 */

/**
 * Pauses execution for the specified number of milliseconds.
 * 
 * This is a utility function that wraps setTimeout in a Promise,
 * allowing for async/await syntax when you need to introduce delays.
 * 
 * @param ms - Number of milliseconds to sleep
 * @returns A promise that resolves after the specified time
 * 
 * @example
 * ```typescript
 * // Simple delay
 * console.log('Starting...');
 * await sleep(2000);
 * console.log('2 seconds later!');
 * 
 * // Animation timing
 * for (let i = 0; i < 5; i++) {
 *   await effects.stamp('countdown::number-' + (5-i) + '.png');
 *   await sleep(1000);
 * }
 * await effects.stamp('countdown::go.png');
 * 
 * // Rate limiting
 * const processItems = async (items: any[]) => {
 *   for (const item of items) {
 *     await processItem(item);
 *     await sleep(100); // 100ms between each item
 *   }
 * };
 * 
 * // Retry with backoff
 * const retryWithDelay = async (operation: () => Promise<any>, maxRetries = 3) => {
 *   for (let attempt = 1; attempt <= maxRetries; attempt++) {
 *     try {
 *       return await operation();
 *     } catch (error) {
 *       if (attempt === maxRetries) throw error;
 *       await sleep(attempt * 1000); // Increasing delay
 *     }
 *   }
 * };
 * ```
 */
export const sleep = (ms: number): Promise<void> => {
    return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Detects the current JavaScript runtime environment.
 * 
 * This function helps determine whether the SDK is running in a browser,
 * Node.js, or Deno environment, which can be useful for conditional logic
 * or environment-specific optimizations.
 * 
 * @returns The detected runtime environment
 * 
 * @example
 * ```typescript
 * const env = runtime();
 * 
 * switch (env) {
 *   case 'browser':
 *     console.log('Running in browser - can use DOM APIs');
 *     document.title = 'Desktop Homunculus Mod';
 *     break;
 *   
 *   case 'nodejs':
 *     console.log('Running in Node.js - can use fs, path, etc.');
 *     const fs = require('fs');
 *     break;
 *   
 *   case 'deno':
 *     console.log('Running in Deno - modern JS runtime');
 *     break;
 * }
 * 
 * // Conditional feature loading
 * const features = {
 *   fileSystem: env === 'nodejs' || env === 'deno',
 *   localStorage: env === 'browser',
 *   websockets: true, // Available in all environments
 * };
 * 
 * // Environment-specific initialization
 * const initializeForEnvironment = () => {
 *   if (runtime() === 'browser') {
 *     // Browser-specific setup
 *     window.addEventListener('beforeunload', cleanup);
 *   } else {
 *     // Server-side setup
 *     process.on('SIGINT', cleanup);
 *   }
 * };
 * ```
 */
export const runtime = () => {
    if (typeof window !== "undefined" && typeof window.EventSource !== "undefined") {
        return "browser" as const;
    } else if (typeof process !== "undefined" && process.versions?.node) {
        return "nodejs" as const;
    } else {
        return "deno" as const;
    }
}