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
export function initialize(): void;
/**
 * @param {number | undefined} [num]
 */
export function get_and_give_f64(num?: number): void;
export function render(): void;
/**
 * @param {HTMLImageElement | undefined} [img]
 */
export function set_tile_image(img?: HTMLImageElement): void;
/**
 * @param {HTMLImageElement | undefined} [img]
 */
export function set_lava_sheet(img?: HTMLImageElement): void;
/**
 * @param {HTMLImageElement | undefined} [img]
 */
export function set_player_image(img?: HTMLImageElement): void;
/**
 * @param {HTMLImageElement | undefined} [img]
 */
export function set_player_image_left(img?: HTMLImageElement): void;
/**
 * @param {HTMLImageElement | undefined} [img]
 */
export function set_player_image_cling(img?: HTMLImageElement): void;
/**
 * @param {HTMLImageElement | undefined} [img]
 */
export function set_player_image_cling_left(img?: HTMLImageElement): void;
/**
 * @param {HTMLImageElement | undefined} [img]
 */
export function set_player_sheet_run_right(img?: HTMLImageElement): void;
/**
 * @param {HTMLImageElement | undefined} [img]
 */
export function set_player_sheet_run_left(img?: HTMLImageElement): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly movement: (a: number) => void;
  readonly initialize: () => void;
  readonly get_and_give_f64: (a: number, b: number) => void;
  readonly render: (a: number) => void;
  readonly stop_movement: (a: number) => void;
  readonly set_tile_image: (a: number) => void;
  readonly set_lava_sheet: (a: number) => void;
  readonly set_player_image: (a: number) => void;
  readonly set_player_image_left: (a: number) => void;
  readonly set_player_image_cling: (a: number) => void;
  readonly set_player_image_cling_left: (a: number) => void;
  readonly set_player_sheet_run_right: (a: number) => void;
  readonly set_player_sheet_run_left: (a: number) => void;
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
