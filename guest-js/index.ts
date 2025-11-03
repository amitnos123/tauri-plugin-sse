import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event';

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:sse|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

type EventCallback = (event: MessageEvent) => void;

class EventSource {
	private readonly eventStartName = "tauri-plugin-sse-";

	private listeners: Record<string, EventCallback> = {};
	
	private url: string;
	
	private _onmessage: EventCallback | null = null;
	private _onerror: EventCallback | null = null;

	set onmessage(callback: EventCallback | null) {
    		this._onmessage = callback;
  	}

  	get onmessage(): EventCallback | null {
    		return this._onmessage;
  	}
	
	set onerror(callback: EventCallback | null) {
    		this._onerror = callback;
  	}

  	get onerror(): EventCallback | null {
    		return this._onerror;
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
    		const unlisten = await listen(`${this.url}-${eventName}`, (e) => {
      			const msgEvent = new MessageEvent(eventName, { data: e.payload });
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
}
