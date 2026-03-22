import { type TransformArgs, Vrm, preferences, repeat, sleep } from "@hmcs/sdk";

const elmerId = "vrm:elmer";
const transform = await preferences.load<TransformArgs>(
    `transform::${elmerId}`,
);
const elmer = await Vrm.spawn(elmerId, {
    transform,
});
const option = {
    repeat: repeat.forever(),
    transitionSecs: 0.5,
} as const;
await elmer.playVrma({
    asset: "vrma:idle-maid",
    ...option,
});
elmer.events().on("state-change", async (e) => {
    if (e.state === "idle") {
        await elmer.playVrma({
            asset: "vrma:idle-maid",
            ...option,
        });
        await sleep(500);
        await elmer.lookAtCursor();
    } else if (e.state === "drag") {
        await elmer.unlook();
        await elmer.playVrma({
            asset: "vrma:grabbed",
            ...option,
            resetSpringBones: true,
        });
    } else if (e.state === "sitting") {
        await elmer.playVrma({
            asset: "vrma:idle-sitting",
            ...option,
        });
        await sleep(500);
        await elmer.lookAtCursor();
    }
});
