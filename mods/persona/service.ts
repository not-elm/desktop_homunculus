import { host, Persona, repeat, sleep, type PersonaSnapshot } from "@hmcs/sdk";

// --- Startup: spawn all auto-spawn personas ---

const personas = await Persona.list();

for (const snapshot of personas) {
    if (snapshot.metadata?.["auto-spawn"] === true) {
        await spawnAndManage(snapshot);
    }
}

// --- Dynamic management via SSE combined stream ---

const streamUrl = host.createUrl("personas/stream");
const source = new EventSource(streamUrl.toString());

source.addEventListener("persona-spawned", async (event) => {
    const data = JSON.parse(event.data) as { personaId: string };
    await manageBehavior(data.personaId);
});

// --- Helpers ---

async function spawnAndManage(snapshot: PersonaSnapshot): Promise<void> {
    const p = new Persona(snapshot.id);
    try {
        await p.spawn();
    } catch {
        // Already spawned — continue to manage behavior
    }
    await manageBehavior(snapshot.id);
}

async function manageBehavior(personaId: string): Promise<void> {
    const p = new Persona(personaId);
    const vrm = p.vrm();

    try {
        await vrm.playVrma({
            asset: "vrma:idle-maid",
            repeat: repeat.forever(),
            transitionSecs: 0.5,
        });
        await vrm.lookAtCursor();
    } catch {
        // VRM not attached or play failed — skip animation
    }

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
