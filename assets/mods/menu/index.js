(async () => {
    Deno.api.Vrm.streamAll(vrm => {
        let webview = null;
        Deno.api.commands.stream("menu::mod::open", async (modWebviewEntity) => {
            await webview?.close();
            webview = modWebviewEntity ? new Deno.api.Webview(modWebviewEntity) : null;
        });
        vrm
            .events()
            .on("pointer-click", async e => {
                if (e.button === "Secondary") {
                    if (await webview?.isClosed()) {
                        webview = null;
                    }
                    if (webview === null) {
                        webview = await Deno.api.Webview.open({
                            source: "menu/index.html",
                            vrm: vrm.entity,
                            parent: await vrm.findBoneEntity("neck"),
                            transform: {
                                translation: [-1.0, 0, 1]
                            },
                            sounds: {
                                "open": "menu/open.mp3",
                            }
                        });
                    } else {
                        await webview?.close();
                        webview = null;
                    }
                }
            });
    });
})();