import {host} from "./host";

/**
 * This class represents the repeat settings for VRMA playback.
 */
export class Repeat {
    readonly args: object;

    private constructor(args: object) {
        this.args = args;
    }

    /**
     * Repeats forever.
     */
    static forever() {
        return new Repeat({
            type: "forever"
        });
    }

    /**
     * Repeats a specified number of times.
     */
    static count(count: number) {
        return new Repeat({
            type: "count",
            count: count,
        });
    }

    /**
     * Does not repeat.
     */
    static never() {
        return new Repeat({
            type: "never"
        });
    }
}

export interface PlayArgs {
    /**
     * Repeat settings for the VRMA playback.
     */
    repeat?: Repeat;
    /**
     * Transition duration in seconds for the VRMA playback.
     */
    transitionSecs?: number;

    /**
     * Whether to wait for the playback to finish.
     */
    waitForCompletion?: boolean;
}

export class Vrma {
    private readonly entity: number;

    constructor(entity: number) {
        this.entity = entity;
    }

    /**
     * Plays the VRMA with the specified options.
     *
     * @param args Optional arguments for playback, such as repeat settings and transition duration.
     */
    async play(args?: PlayArgs) {
        await host.put(host.createUrl(`vrma/${this.entity}/play`), {
            repeat: args?.repeat?.args,
            transitionSecs: args?.transitionSecs,
            waitForCompletion: args?.waitForCompletion,
        });
    }

    /**
     * Stops the VRMA playback.
     */
    async stop() {
        await host.put(host.createUrl(`vrma/${this.entity}/stop`));
    }
}