import {host} from "./host";

/**
 * Provides access to the application API.
 */
export namespace app {
    /**
     * Exits the application without any problems.
     */
    export const exit = async () => {
        await host.post(host.createUrl("app/exit"));
    }
}