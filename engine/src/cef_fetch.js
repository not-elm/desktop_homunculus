(function () {
    if (cef.fetch) {
        return;
    }
    const pending = new Map();
    let listenerRegistered = false;

    function registerListener() {
        if (listenerRegistered) return;
        listenerRegistered = true;
        cef.listen("cef_fetch_response", function (payload) {
            payload = JSON.parse(payload)
            if (!payload || !payload.id) return;
            const entry = pending.get(payload.id);
            if (!entry) return;
            pending.delete(payload.id);
            if (entry.signal && entry.abortHandler) {
                entry.signal.removeEventListener("abort", entry.abortHandler);
            }
            if (payload.error) {
                entry.reject(new TypeError(payload.error));
                return;
            }
            entry.resolve(new CefResponse(payload));
        });
    }

    function makeAbortError() {
        try {
            return new DOMException("Aborted", "AbortError");
        } catch (e) {
            const err = new Error("Aborted");
            err.name = "AbortError";
            return err;
        }
    }

    function generateId() {
        if (typeof crypto !== "undefined" && crypto.randomUUID) {
            return crypto.randomUUID();
        }
        return "req_" + Math.random().toString(16).slice(2) + "_" + Date.now();
    }

    function bytesToBase64(bytes) {
        let bin = "";
        for (let i = 0; i < bytes.length; i++) {
            bin += String.fromCharCode(bytes[i]);
        }
        return btoa(bin);
    }

    function base64ToBytes(base64) {
        if (!base64) return new Uint8Array(0);
        const bin = atob(base64);
        const bytes = new Uint8Array(bin.length);
        for (let i = 0; i < bin.length; i++) {
            bytes[i] = bin.charCodeAt(i);
        }
        return bytes;
    }

    function encodeBody(body) {
        if (body == null) return null;
        if (typeof body === "string") {
            return bytesToBase64(new TextEncoder().encode(body));
        }
        if (body instanceof ArrayBuffer) {
            return bytesToBase64(new Uint8Array(body));
        }
        if (ArrayBuffer.isView(body)) {
            return bytesToBase64(new Uint8Array(body.buffer, body.byteOffset, body.byteLength));
        }
        throw new TypeError("Unsupported body type");
    }

    function normalizeHeaders(init) {
        const headers = new CefHeaders(init);
        const pairs = [];
        headers.forEach((value, key) => {
            pairs.push([key, value]);
        });
        return pairs;
    }

    function CefHeaders(init) {
        this._map = new Map();
        if (!init) return;
        if (init instanceof CefHeaders) {
            init.forEach((v, k) => this.append(k, v));
            return;
        }
        if (Array.isArray(init)) {
            for (const pair of init) {
                if (pair && pair.length >= 2) {
                    this.append(pair[0], pair[1]);
                }
            }
            return;
        }
        if (typeof init === "object") {
            for (const key in init) {
                this.append(key, init[key]);
            }
        }
    }

    CefHeaders.prototype.append = function (name, value) {
        const key = String(name).toLowerCase();
        const existing = this._map.get(key);
        if (existing) {
            this._map.set(key, existing + ", " + String(value));
        } else {
            this._map.set(key, String(value));
        }
    };
    CefHeaders.prototype.set = function (name, value) {
        this._map.set(String(name).toLowerCase(), String(value));
    };
    CefHeaders.prototype.get = function (name) {
        return this._map.get(String(name).toLowerCase()) || null;
    };
    CefHeaders.prototype.has = function (name) {
        return this._map.has(String(name).toLowerCase());
    };
    CefHeaders.prototype.forEach = function (callback) {
        this._map.forEach((value, key) => callback(value, key));
    };
    CefHeaders.prototype.entries = function () {
        return this._map.entries();
    };

    function CefResponse(payload) {
        this.status = payload.status || 0;
        this.ok = !!payload.ok;
        this.statusText = payload.statusText || "";
        this.url = payload.url || "";
        this.headers = new CefHeaders(payload.headers || []);
        this._bodyBase64 = payload.bodyBase64 || "";
    }

    CefResponse.prototype.text = function () {
        const bytes = base64ToBytes(this._bodyBase64);
        return Promise.resolve(new TextDecoder().decode(bytes));
    };
    CefResponse.prototype.json = function () {
        return this.text().then((text) => JSON.parse(text));
    };
    CefResponse.prototype.arrayBuffer = function () {
        const bytes = base64ToBytes(this._bodyBase64);
        return Promise.resolve(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength));
    };

    cef.Headers = CefHeaders;
    cef.Response = CefResponse;

    cef.fetch = function (url, init) {
        registerListener();
        return new Promise((resolve, reject) => {
            if (!url) {
                reject(new TypeError("URL is required"));
                return;
            }
            const options = init || {};
            const id = generateId();
            const method = (options.method || "GET").toString().toUpperCase();
            const headers = normalizeHeaders(options.headers);
            let bodyBase64 = null;
            try {
                bodyBase64 = encodeBody(options.body);
            } catch (err) {
                reject(err);
                return;
            }
            const signal = options.signal;
            let aborted = false;
            const abortHandler = function () {
                if (aborted) return;
                aborted = true;
                pending.delete(id);
                cef.emit({type: "abort", id: id});
                reject(makeAbortError());
            };
            if (signal) {
                if (signal.aborted) {
                    abortHandler();
                    return;
                }
                signal.addEventListener("abort", abortHandler, {once: true});
            }
            pending.set(id, {resolve, reject, signal, abortHandler});
            cef.emit({
                type: "request",
                id: id,
                url: String(url),
                method: method,
                headers: headers,
                bodyBase64: bodyBase64,
                redirect: "follow"
            });
        });
    };
})();