/* tslint:disable */
/* eslint-disable */
/**
*/
export function stop_up(): void;
/**
*/
export function stop_down(): void;
/**
*/
export function stop_left(): void;
/**
*/
export function stop_right(): void;
/**
*/
export function stop_jumping(): void;
/**
*/
export function move_right(): void;
/**
*/
export function move_left(): void;
/**
*/
export function move_up(): void;
/**
*/
export function move_down(): void;
/**
*/
export function jump(): void;
/**
*/
export function initialize(): void;
/**
*/
export function update(): void;
/**
*/
export function render(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly initialize: () => void;
  readonly render: (a: number) => void;
  readonly stop_up: () => void;
  readonly stop_down: () => void;
  readonly stop_left: () => void;
  readonly stop_right: () => void;
  readonly stop_jumping: () => void;
  readonly jump: () => void;
  readonly update: () => void;
  readonly move_right: () => void;
  readonly move_left: () => void;
  readonly move_up: () => void;
  readonly move_down: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
