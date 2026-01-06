/* tslint:disable */
/* eslint-disable */

export function beginConnection(): void;

export function createRequest(): void;

export function createResponse(): void;

export function enableAI(): void;

export function handleAIMove(): void;

export function handleDataIn(data: string): void;

export function handleKeyDown(key: string): void;

export function handleMouseClick(mouseX: number, mouseY: number): void;

export function render(): void;

export function setHook(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly beginConnection: () => void;
  readonly createRequest: () => void;
  readonly createResponse: () => void;
  readonly enableAI: () => void;
  readonly handleAIMove: () => void;
  readonly handleDataIn: (a: number, b: number) => void;
  readonly handleKeyDown: (a: number, b: number) => void;
  readonly handleMouseClick: (a: number, b: number) => void;
  readonly render: () => void;
  readonly setHook: () => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
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
