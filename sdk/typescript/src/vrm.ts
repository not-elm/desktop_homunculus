import {Transform} from "./math";
import {host} from "./host";
import {EventSource} from "eventSource";
import {entities} from "./entities";
import {Vrma} from "./vrma";

export interface SpawnVrmOptions {
    transform?: Partial<Transform>;
}

export interface SpeakOnVoiceVoxOptions {
    /**
     * The voice vox speaker ID.
     */
    speaker?: number;
    /**
     * The pause duration in seconds between sentences.
     */
    pause?: number;
    /**
     * If true, the method will wait for the speech to complete.
     */
    waitForCompletion?: boolean;
    /**
     * The speed scale of the speech.
     * The default value is 1.0.
     */
    subtitle?: SubtitleOptions,
}

export interface SubtitleOptions {
    /**
     * The mod asset ID of the font to use for the subtitle text.
     */
    font?: string;
    /**
     * The font size of the subtitle text.
     */
    fontSize?: number;
    /**
     * The color of the subtitle text.
     * The values are in the range of 0 to 1.
     * The format is [r, g, b, a].
     */
    color?: [number, number, number, number];
}

export interface VrmPointerEvent {
    /**
     * The cursor position in the global viewport.
     */
    globalViewport: [number, number];
}

export interface VrmDragEvent extends VrmPointerEvent {
    /**
     * The change in cursor position since the last event.
     */
    delta: [number, number];
}

export type Button = "Primary" | "Secondary" | "Middle";

export interface VrmMouseEvent extends VrmPointerEvent {
    /**
     * The button that was pressed or released.
     */
    button: Button;
}

export interface VrmStateChangeEvent {
    /**
     * The new state of the VRM.
     */
    state: string;
}

export type EventMap = {
    "drag-start": VrmPointerEvent;
    "drag": VrmDragEvent;
    "drag-end": VrmPointerEvent;
    "pointer-press": VrmMouseEvent;
    "pointer-click": VrmMouseEvent;
    "pointer-release": VrmMouseEvent;
    "pointer-over": VrmPointerEvent;
    "pointer-out": VrmPointerEvent;
    "pointer-cancel": VrmPointerEvent;
    "state-change": VrmStateChangeEvent;
};

export interface VrmMetadata {
    name: string;
    entity: number;
}

export type Bones =
    "hips"
    | "spine"
    | "chest"
    | "neck"
    | "head"
    | "leftShoulder"
    | "leftArm"
    | "leftForeArm"
    | "leftHand"
    | "rightShoulder"
    | "rightArm"
    | "rightForeArm"
    | "rightHand"
    | "leftUpLeg"
    | "leftLeg"
    | "leftFoot"
    | "rightUpLeg"
    | "rightLeg"
    | "rightFoot";

export class VrmEventSource implements Disposable {
    constructor(readonly eventSource: EventSource) {

    }

    /**
     * Registers an event listener for the specified event type.
     */
    on<K extends keyof EventMap>(event: K, callback: (event: EventMap[K]) => (void | Promise<void>)) {
        this.eventSource.addEventListener(event, e => {
            callback(JSON.parse(e.data) as EventMap[K])
        });
    }

    /**
     * Closes the EventSource connection.
     */
    close() {
        this.eventSource.close();
    }

    [Symbol.dispose]() {
        this.eventSource.close();
    }
}

export class Vrm {
    constructor(readonly entity: number) {

    }

    /**
     * Returns an EventSource for receiving events related to this VRM entity.
     */
    events(): VrmEventSource {
        const url = host.createUrl(`vrm/${this.entity}/events`);
        return new VrmEventSource(new EventSource(url));
    }

    /**
     * Returns the current state of the VRM.
     */
    async state() {
        const response = await this.fetch("state");
        const json = await response.json() as { state: string };
        return json.state;
    }

    /**
     * Sets the state of the VRM.
     *
     * @param state The new state to set.
     */
    async setState(state: string): Promise<void> {
        await this.put("state", {state});
    }

    /**
     * Returns the name of the VRM avatar.
     */
    async name(): Promise<string> {
        return await entities.name(this.entity);
    }

    /**
     * Finds the entity ID of a bone by its name.
     */
    async findBoneEntity(bone: Bones): Promise<number> {
        return await entities.findByName(bone, {
            root: this.entity,
        });
    }

    /**
     * Returns a VRMA instance.
     * If the VRMA does not exist, spawn a new one and return it.
     *
     * @param source The vrma path relative to the mods directory.
     */
    async vrma(source: string) {
        const response = await host.get(host.createUrl(`vrm/${this.entity}/vrma`, {
            source,
        }));
        const vrmaEntity = Number(await response.json());
        return new Vrma(vrmaEntity);
    }

    /**
     * Speaks the given text using VoiceVox.
     * Please make sure that the VoiceVox server is running.
     * Note that if the server isn't running, no error will be thrown and nothing will happen.
     */
    async speakOnVoiceVox(sentences: string[] | string, options?: SpeakOnVoiceVoxOptions) {
        const response = await this.post("speech/voicevox", {
            sentences: (typeof sentences === "string") ? [sentences] : sentences,
            ...options
        });
        return response.body!!.pipeThrough(new TextDecoderStream());
    }

    /**
     * Looks at the mouse cursor.
     */
    async lookAtCursor() {
        await this.put("look/cursor");
    }

    /**
     * Sets the VRM's look-at target to a specific entity.
     *
     * @param target The entity ID to look at.
     */
    async lookAtTarget(target: number) {
        await this.put(`look/target/${target}`);
    }

    /**
     * Disables the VRM's look-at functionality.
     */
    async unlook() {
        await this.delete("look");
    }

    /**
     * Spawns a new VRM instance from the given mod asset ID.
     */
    static async spawn(asset: string, options?: SpawnVrmOptions): Promise<Vrm> {
        const response = await host.post(host.createUrl("vrm"), {
            asset,
            ...options
        });
        return new Vrm(Number(await response.text()));
    }

    /**
     * Finds a VRM instance by its name.
     *
     * @param vrmName VRM avatar name
     */
    static async findByName(vrmName: string): Promise<Vrm> {
        return new Vrm(Number(await entities.findByName(vrmName)));
    }

    /**
     * Waits for a VRM instance to be spawned and initialized by its name.
     *
     * @param vrmName VRM avatar name
     */
    static async waitLoadByName(vrmName: string): Promise<Vrm> {
        const response = await host.get(host.createUrl("vrm/wait-load", {
            name: vrmName,
        }));
        return new Vrm(Number(await response.json()));
    }

    static async findAllMetadata(): Promise<VrmMetadata[]> {
        const response = await host.get(host.createUrl("vrm/all"));
        return await response.json();
    }

    static streamAllMetadata(
        f: (vrm: VrmMetadata) => (void | Promise<void>)
    ): EventSource {
        const es = new EventSource(host.createUrl("vrm/all?stream=true"));
        es.addEventListener("message", (e) => {
            f(JSON.parse(e.data));
        });
        return es;
    }

    /**
     * This method streams all currently existing VRM instances and any VRM instances that will be created in the future.
     * @param f
     */
    static streamAll(
        f: (vrm: Vrm) => (void | Promise<void>)
    ): EventSource {
        return Vrm.streamAllMetadata(metadata => {
            f(new Vrm(metadata.entity));
        });
    }

    /**
     * Returns all VRM instances that are currently loaded.
     */
    static async findAll(): Promise<Vrm[]> {
        const allMetadata = await Vrm.findAllMetadata();
        return allMetadata.map(metadata => new Vrm(metadata.entity));
    }

    /**
     * Returns the VRM instance that is specified as the caller of this Webview.
     * If no caller is specified, it returns undefined.
     *
     * This method helps create UI that is specific to a VRM, such as a settings screen.
     */
    static caller(): Vrm | undefined {
        //@ts-ignore
        const entity: number | undefined = window.CALLER_VRM_ENTITY;
        return entity !== undefined ? new Vrm(entity) : undefined;
    }

    private async fetch(path: string): Promise<Response> {
        return await host.get(host.createUrl(`vrm/${this.entity}/${path}`));
    }

    private async post(path: string, body: object) {
        return await host.post(host.createUrl(`vrm/${this.entity}/${path}`), body);
    }

    private async put(path: string, body?: any) {
        await host.put(host.createUrl(`vrm/${this.entity}/${path}`), body);
    }

    private async delete(path: string) {
        await host.deleteMethod(host.createUrl(`vrm/${this.entity}/${path}`));
    }
}

