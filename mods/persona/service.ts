import { Persona, repeat, sleep, type PersonaSnapshot } from "@hmcs/sdk";

// --- Startup: spawn all auto-spawn personas ---

const personas = await Persona.list();

for (const snapshot of personas) {
    const autoSpawn = snapshot.metadata?.["auto-spawn"];

    // Migration: first run, set auto-spawn=true for existing personas without the key
    if (autoSpawn === undefined) {
        const p = new Persona(snapshot.id);
        await p.patch({ metadata: { ...snapshot.metadata, "auto-spawn": true } });
        await spawnAndManage(snapshot);
        continue;
    }

    if (autoSpawn === true) {
        await spawnAndManage(snapshot);
    }
}

// --- Helper: spawn a persona and set up behavior ---

async function spawnAndManage(snapshot: PersonaSnapshot): Promise<void> {
    const p = new Persona(snapshot.id);

    try {
        await p.spawn();
    } catch {
        // Already spawned or other error — continue to manage
    }

    if (snapshot.vrmAssetId) {
        try {
            const vrm = await p.attachVrm(snapshot.vrmAssetId);
            await vrm.playVrma({
                asset: "vrma:idle-maid",
                repeat: repeat.forever(),
                transitionSecs: 0.5,
            });
            await vrm.lookAtCursor();
        } catch {
            // VRM attach failed — persona spawned without VRM
        }
    }

    // Subscribe to state changes
    p.events().on("state-change", async (e) => {
        const v = p.vrm();
        const option = {
            repeat: repeat.forever(),
            transitionSecs: 0.5,
        } as const;

        if (e.state === "idle") {
            await v.playVrma({ asset: "vrma:idle-maid", ...option });
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
            await v.playVrma({ asset: "vrma:idle-sitting", ...option });
            await sleep(500);
            await v.lookAtCursor();
        }
    });
}
