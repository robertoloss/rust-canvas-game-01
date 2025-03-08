/* tslint:disable */
/* eslint-disable */
/**
 * @param {number} key_code
 */
export function movement(key_code: number): void;
/**
 * @param {number} key_code
 */
export function stop_movement(key_code: number): void;
/**
 * @param {string} name
 * @param {boolean} sheet
 * @param {HTMLImageElement | undefined} [img]
 */
export function set_image(name: string, sheet: boolean, img?: HTMLImageElement): void;
/**
 * @param {number | undefined} [num]
 */
export function get_and_give_f64(num?: number): void;
export function initialize(): void;
export function render(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly movement: (a: number) => void;
  readonly stop_movement: (a: number) => void;
  readonly set_image: (a: number, b: number, c: number, d: number) => void;
  readonly get_and_give_f64: (a: number, b: number) => void;
  readonly initialize: () => void;
  readonly render: () => Array;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __externref_table_alloc: () => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
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
