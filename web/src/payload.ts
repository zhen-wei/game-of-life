export const enum MsgDataEnum {
    init,
    render,
    toggle,
    pause,
}

export interface InitPayload {
    width: number;
    height: number;
    cellSize: number;
    canvas: OffscreenCanvas;
    deadColor?: string;
    aliveColor?: string;
}

export interface TogglePayload {
    row: number;
    col: number;
}

export interface MsgDataMap {
    [MsgDataEnum.init]: InitPayload;
    [MsgDataEnum.render]: number;
    [MsgDataEnum.toggle]: TogglePayload;
    [MsgDataEnum.pause]: boolean;
}

export type MsgData<T extends MsgDataEnum = MsgDataEnum> = T extends MsgDataEnum ? { type: T; payload: MsgDataMap[T] } : never;

export function createMsgData<T extends MsgDataEnum = MsgDataEnum>(type: T, payload: MsgDataMap[T]): MsgData<T> {
    return {
        type,
        payload,
    } as MsgData<T>;
}
