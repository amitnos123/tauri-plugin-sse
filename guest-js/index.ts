import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event';

export enum State {
	Connecting,
	Open,
	Closed
}

export interface MessageEvent {
  id?: string;
  type: string;
  data: any;
}

type EventCallback = (event: MessageEvent) => void;
type EventUnlisten = () => void;

export class EventSource {
	private readonly eventStartName = "tauri-plugin-sse-";

	private listeners: Record<string, EventCallback> = {};
	private unlistenMap: Record<string, EventUnlisten> = {};
	
	private url: string;
	
	private _onmessage: EventCallback | null = null;
	private _onerror: EventCallback | null = null;
	private _onopen: EventCallback | null = null;
	private _state: State;
	
  	get onmessage(): EventCallback | null {
    		return this._onmessage;
  	}

	set onmessage(callback: EventCallback | null) {
    		this._onmessage = callback;

			this.syncSetListen("on_message", callback, "add_on_message_sse")
  	}

  	get onopen(): EventCallback | null {
    		return this._onopen;
  	}

	set onopen(callback: EventCallback | null) {
    		this._onopen = callback;

			this.syncSetListen("on_open", callback, "open_sse");
  	}
	
	set onerror(callback: EventCallback | null) {
    		this._onerror = callback;

			this.syncSetListen("on_error", callback, "on_error_sse");
  	}

  	get onerror(): EventCallback | null {
    		return this._onerror;
  	}

	get state(): State {
	    return this._state;
	}
	
	/* Add an event listener
	and saved unlisten callback inside this.unlistenMap[name]
	Does this Async block
	*/
	private syncSetListen(name: string, callback: EventCallback | null, command: string) {
		  // Run async code in background
		(async () => {
			try {
			let is_success : boolean = await invoke<boolean>('plugin:sse|' + command, {url: this.url})
			.then((r : boolean) => r);
			if (is_success) {
				const safeUrl = this.sanitizeUrl(this.url);
				const unlisten = await listen<MessageEvent>(
					`${this.eventStartName}${safeUrl}-${name}`,
					(e) => {
						callback?.(e.payload);
					}
				);

				this.unlistenMap[name] = unlisten;
			}
				return {is_success: is_success, name: name};
			} catch (err) {
				console.error(`Failed to set listener for ${name}:`, err);
			}
			})();
	}
	
	private sanitizeUrl(url: string): string {
		return url
			.replace(/:\/\//g, '__')   // replace :// 
			.replace(/[^a-zA-Z0-9\-/_:]/g, '_'); // replace other disallowed chars
	}

	constructor(url: string) {
		this.url = url;
		this._state = State.Connecting;

		this.open().then((r : boolean) => (this._state = r ? State.Open : State.Closed));
	}

	async open() : Promise<boolean> {
		let r : boolean = await invoke<boolean>('plugin:sse|open_sse', {url: this.url});
		if (r) {
			this._state = State.Open;
		}
		return r;
	}

	/** Add named listener */
  	async addEventListener(eventName: string, callback: EventCallback) : Promise<boolean> {
		let is_success = await invoke<boolean>('plugin:sse|add_event_listener', {url: this.url, name: eventName})
				.then((r : boolean) => r);
		if (is_success) {
			this.listeners[eventName] = callback;

			// If already listening, remove previous
			if (this.unlistenMap[eventName]) {
				await this.unlistenMap[eventName]!();
			}

			const safeUrl = this.sanitizeUrl(this.url);
			// Listen to Tauri event
			const unlisten = await listen<MessageEvent>(`${this.eventStartName}${safeUrl}-${eventName}`, (e) => {
				callback(e.payload);
			});

			this.unlistenMap[eventName] = unlisten;
		}
		return is_success;
  	}

  	/** Remove named listener */
  	async removeEventListener(eventName: string) : Promise<boolean> {
		let is_success = await invoke<boolean>('plugin:sse|remove_event_listener', {url: this.url, name: eventName})
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

	async close() : Promise<boolean> {
		let r : boolean = await invoke<boolean>('plugin:sse|close_sse', {url: this.url});
		if (r) {
			this._state = State.Closed;
		}
		return r;
	}
}
