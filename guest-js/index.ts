import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event';

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:sse|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export interface MessageEvent {
  type: string;
  data: any;
}

type EventCallback = (event: MessageEvent) => void;

export class EventSource {
	private readonly eventStartName = "tauri-plugin-sse-";

	private listeners: Record<string, EventCallback> = {};
	
	private url: string;
	
	private _onmessage: EventCallback | null = null;
	private _onerror: EventCallback | null = null;
	private _onopen: EventCallback | null = null;

  	get onmessage(): EventCallback | null {
    		return this._onmessage;
  	}

	set onmessage(callback: EventCallback | null) {
    		this._onmessage = callback;

			syncSetListen("message", callback)
  	}

  	get onopen(): EventCallback | null {
    		return this._onopen;
  	}

	set onopen(callback: EventCallback | null) {
    		this._onopen = callback;

			syncSetListen("open", callback)
  	}
	
	set onerror(callback: EventCallback | null) {
    		this._onerror = callback;

			syncSetListen("error", callback)
  	}

  	get onerror(): EventCallback | null {
    		return this._onerror;
  	}

	private syncSetListen(name: string, callback?: EventCallback) {
		  // Run async code in background
		  (async () => {
		    try {
		      const unlisten = await listen(
		        `${this.eventStartName}${this.url}-${name}`,
		        (e) => {
					const msgEvent: MessageEvent = { type: eventName, data: e.payload };
		          	callback?.(msgEvent);
		        }
		      );
		
		      this.unlistenMap[name] = unlisten;
		    } catch (err) {
		      console.error(`Failed to set listener for ${name}:`, err);
		    }
		  })();
	}
	
	constructor(url: string) {
		this.url = url;
	}

	/** Add named listener */
  	async addEventListener(eventName: string, callback: EventCallback) {
    		this.listeners[eventName] = callback;

    		// If already listening, remove previous
    		if (this.unlistenMap[eventName]) {
      			await this.unlistenMap[eventName]!();
    		}

    		// Listen to Tauri event
    		const unlisten = await listen(`${eventStartName}${this.url}-${eventName}`, (e) => {
      			const msgEvent: MessageEvent = { type: eventName, data: e.payload };
	      		callback(msgEvent);
	    	});

    	this.unlistenMap[eventName] = unlisten;
  	}

  	/** Remove named listener */
  	async removeEventListener(eventName: string) {
    		delete this.listeners[eventName];

    		const unlisten = this.unlistenMap[eventName];
    		if (unlisten) {
      			await unlisten();
      			delete this.unlistenMap[eventName];
    		}
  	}

	close() {
		
	}
}
