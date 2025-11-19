export declare enum State {
    Connecting = 0,
    Open = 1,
    Closed = 2
}
export interface MessageEvent {
    id?: string;
    type: string;
    data: any;
}
type EventCallback = (event: MessageEvent) => void;
export declare class EventSource {
    private readonly eventStartName;
    private listeners;
    private unlistenMap;
    private url;
    private _onmessage;
    private _onerror;
    private _onopen;
    private _state;
    get onmessage(): EventCallback | null;
    set onmessage(callback: EventCallback | null);
    get onopen(): EventCallback | null;
    set onopen(callback: EventCallback | null);
    set onerror(callback: EventCallback | null);
    get onerror(): EventCallback | null;
    get state(): State;
    private syncSetListen;
    constructor(url: string);
    open(): Promise<Boolean>;
    /** Add named listener */
    addEventListener(eventName: string, callback: EventCallback): Promise<boolean>;
    /** Remove named listener */
    removeEventListener(eventName: string): Promise<Boolean>;
    close(): Promise<Boolean>;
}
export {};
