(async () => {
    const elmer = await Deno.api.Vrm.spawn("elmer/Elmer.vrm");
    const idle = await elmer.vrma("elmer/idle.vrma");
    await idle.play({
        repeat: Deno.api.Repeat.forever(),
    });
    elmer.events().on("state-change", async (e) => {
        if (e.state === "idle") {
            await idle.play({
                repeat: Deno.api.Repeat.forever(),
                transitionSecs: 0.5,
            });
        }
    });
})();
