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
                        const webviewWidth = 500;
                        const webviewHeight = 600;
                        webview = await Deno.api.Webview.open({
                            source: "menu/index.html",
                            caller: vrm.entity,
                            position: {
                                vrm: vrm.entity,
                                offset: [-webviewWidth - 100, -180],
                                bone: "head",
                                tracking: true,
                            },
                            transparent: true,
                            showToolbar: false,
                            resolution: [webviewWidth, webviewHeight],
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