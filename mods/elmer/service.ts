import { Persona, repeat, sleep } from "@hmcs/sdk";

let elmer;
try {
    elmer = await Persona.create({ id: "elmer", name: "Elmer" });
} catch {
    elmer = await Persona.load("elmer");
}

const vrm = await elmer.attachVrm("vrm:elmer");
const option = {
    repeat: repeat.forever(),
    transitionSecs: 0.5,
} as const;
await vrm.playVrma({
    asset: "vrma:idle-maid",
    ...option,
});
elmer.events().on("state-change", async (e) => {
    const v = elmer.vrm();
    if (e.state === "idle") {
        await v.playVrma({
            asset: "vrma:idle-maid",
            ...option,
        });
        await sleep(500);
        await v.lookAtCursor();
    } else if (e.state === "drag") {
        await v.unlook();
        await v.playVrma({
            asset: "vrma:grabbed",
            ...option,
            resetSpringBones: true,
        });
    } else if (e.state === "sitting") {
        await v.playVrma({
            asset: "vrma:idle-sitting",
            ...option,
        });
        await sleep(500);
        await v.lookAtCursor();
    }
});
