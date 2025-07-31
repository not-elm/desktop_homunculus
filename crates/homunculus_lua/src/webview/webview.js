Object.defineProperty(window, "emitEvent", {
    value: (id, args) => {
        window.__FLURX__.emit("script-event", {
            id,
            args,
        });
    },
    writable: false,
    configurable: true,
});