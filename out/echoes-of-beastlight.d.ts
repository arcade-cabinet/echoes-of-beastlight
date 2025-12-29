/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: (a: number, b: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__h2ee19fe8e06f79e8: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h0a84befd12f53754: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hf763899942d8357a: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__he6e1201ad2cc181f: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hc9238f704e7a31b6: (a: number, b: number, c: any, d: any) => void;
  readonly wasm_bindgen__closure__destroy__h05fe2d273f268fd9: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h17b1bafa6fde1bf8: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h5cce09d27d78a4d3: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h7b41c7aa90890609: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h6d1ed0fa8ca8dafd: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__ha38f8c1ffac97e21: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
