import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

var State;
(function (State) {
    State[State["Connecting"] = 0] = "Connecting";
    State[State["Open"] = 1] = "Open";
    State[State["Closed"] = 2] = "Closed";
})(State || (State = {}));
class EventSource {
    get onmessage() {
        return this._onmessage;
    }
    set onmessage(callback) {
        this._onmessage = callback;
        this.syncSetListen("on_message", callback, "add_on_message_sse");
    }
    get onopen() {
        return this._onopen;
    }
    set onopen(callback) {
        this._onopen = callback;
        this.syncSetListen("on_open", callback, "open_sse");
    }
    set onerror(callback) {
        this._onerror = callback;
        this.syncSetListen("on_error", callback, "on_error_sse");
    }
    get onerror() {
        return this._onerror;
    }
    get state() {
        return this._state;
    }
    /* Add an event listener
    and saved unlisten callback inside this.unlistenMap[name]
    Does this Async block
    */
    syncSetListen(name, callback, command) {
        // Run async code in background
        (async () => {
            try {
                let is_success = await invoke('plugin:sse|' + command, { url: this.url })
                    .then((r) => r);
                if (is_success) {
                    const safeUrl = this.sanitizeUrl(this.url);
                    const unlisten = await listen(`${this.eventStartName}${safeUrl}-${name}`, (e) => {
                        callback?.(e.payload);
                    });
                    this.unlistenMap[name] = unlisten;
                }
                return { is_success: is_success, name: name };
            }
            catch (err) {
                console.error(`Failed to set listener for ${name}:`, err);
            }
        })();
    }
    sanitizeUrl(url) {
        return url
            .replace(/:\/\//g, '__') // replace :// 
            .replace(/[^a-zA-Z0-9\-/_:]/g, '_'); // replace other disallowed chars
    }
    constructor(url) {
        this.eventStartName = "tauri-plugin-sse-";
        this.listeners = {};
        this.unlistenMap = {};
        this._onmessage = null;
        this._onerror = null;
        this._onopen = null;
        this.url = url;
        this._state = State.Connecting;
        this.open().then((r) => (this._state = r ? State.Open : State.Closed));
    }
    async open() {
        let r = await invoke('plugin:sse|open_sse', { url: this.url });
        if (r) {
            this._state = State.Open;
        }
        return r;
    }
    /** Add named listener */
    async addEventListener(eventName, callback) {
        let is_success = await invoke('plugin:sse|add_event_listener', { url: this.url, name: eventName })
            .then((r) => r);
        if (is_success) {
            this.listeners[eventName] = callback;
            // If already listening, remove previous
            if (this.unlistenMap[eventName]) {
                await this.unlistenMap[eventName]();
            }
            const safeUrl = this.sanitizeUrl(this.url);
            // Listen to Tauri event
            const unlisten = await listen(`${this.eventStartName}${safeUrl}-${eventName}`, (e) => {
                callback(e.payload);
            });
            this.unlistenMap[eventName] = unlisten;
        }
        return is_success;
    }
    /** Remove named listener */
    async removeEventListener(eventName) {
        let is_success = await invoke('plugin:sse|remove_event_listener', { url: this.url, name: eventName });
        if (is_success) {
            delete this.listeners[eventName];
            const unlisten = this.unlistenMap[eventName];
            if (unlisten) {
                await unlisten();
                delete this.unlistenMap[eventName];
            }
        }
        return is_success;
    }
    async close() {
        let r = await invoke('plugin:sse|close_sse', { url: this.url });
        if (r) {
            this._state = State.Closed;
        }
        return r;
    }
}

export { EventSource, State };
