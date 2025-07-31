import {host} from "./host";

/**
 * Scripts API namespace for executing JavaScript code from mod assets.
 *
 * Provides functionality to dynamically execute JavaScript code that is stored
 * as mod assets. This enables powerful scripting capabilities for mods, allowing
 * them to run complex logic, automation, and dynamic behaviors.
 *
 * Key features:
 * - Execute JavaScript from mod asset files
 * - Safe script execution in isolated contexts
 * - Integration with other SDK functionality
 * - Support for complex mod behaviors and automation
 *
 * @example
 * ```typescript
 * // Execute a JavaScript file from a mod
 * await scripts.callJavascript('automation-mod::daily-routine.js');
 *
 * // Execute scripts conditionally
 * const currentHour = new Date().getHours();
 * if (currentHour === 9) {
 *   await scripts.callJavascript('morning-routine::greet-user.js');
 * } else if (currentHour === 17) {
 *   await scripts.callJavascript('evening-routine::wrap-up.js');
 * }
 *
 * // Execute scripts in response to events
 * const vrm = await Vrm.findByName('Assistant');
 * const events = vrm.events();
 *
 * events.on('pointer-click', async () => {
 *   await scripts.callJavascript('interactions::handle-click.js');
 * });
 *
 * // Chain script execution
 * await scripts.callJavascript('setup::initialize.js');
 * await sleep(1000);
 * await scripts.callJavascript('setup::configure.js');
 * await scripts.callJavascript('setup::finalize.js');
 * ```
 */
export namespace scripts {
    /**
     * Executes a JavaScript file from a mod asset.
     *
     * This function loads and executes JavaScript code stored in mod assets,
     * allowing for dynamic scripting and automation within the Desktop Homunculus
     * environment. The script runs in the context of the application and has
     * access to all SDK functionality.
     *
     * @param source - The script path relative to `assets/mods` directory.
     *
     * @example
     * ```typescript
     * // Execute a simple automation script
     * await scripts.callJavascript('daily-tasks::morning-setup.js');
     *
     * // Execute AI behavior scripts
     * await scripts.callJavascript('ai-behaviors::emotional-response.js');
     *
     * // Execute complex interaction handlers
     * await scripts.callJavascript('interactions::advanced-chat.js');
     *
     * // Execute maintenance scripts
     * await scripts.callJavascript('maintenance::cleanup-old-data.js');
     *
     * // Build a script execution pipeline
     * const executionPipeline = [
     *   'pipeline::validate-environment.js',
     *   'pipeline::load-configuration.js',
     *   'pipeline::setup-characters.js',
     *   'pipeline::start-interactions.js'
     * ];
     *
     * for (const scriptAsset of executionPipeline) {
     *   console.log(`Executing: ${scriptAsset}`);
     *   await scripts.callJavascript(scriptAsset);
     *   console.log(`Completed: ${scriptAsset}`);
     * }
     *
     * // Conditional script execution based on application state
     * const vrms = await Vrm.findAll();
     * if (vrms.length === 0) {
     *   await scripts.callJavascript('startup::spawn-default-character.js');
     * } else {
     *   await scripts.callJavascript('startup::restore-character-states.js');
     * }
     *
     * // Error handling for script execution
     * try {
     *   await scripts.callJavascript('risky-script::experimental.js');
     * } catch (error) {
     *   console.error('Script execution failed:', error);
     *   // Fallback to safe script
     *   await scripts.callJavascript('fallback::safe-mode.js');
     * }
     * ```
     */
    export const callJavascript = async (source: string): Promise<void> => {
        await host.post(host.createUrl(`scripts/js`), {
            source,
        });
    }
}