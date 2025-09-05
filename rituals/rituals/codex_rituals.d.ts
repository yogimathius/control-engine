/* tslint:disable */
/* eslint-disable */
/**
 * Shadow Integration Ritual - Rust/WASM Implementation
 */
export function shadow_integration_ritual(): any;
/**
 * Energy Attunement Ritual - Balances elemental energies
 */
export function energy_attunement_ritual(): any;
/**
 * Void Contemplation Ritual - Deep meditation practice
 */
export function void_contemplation_ritual(): any;
/**
 * Archetype Invocation - General archetype activation
 */
export function archetype_invocation_ritual(): any;
/**
 * Test function to verify WASM module loading
 */
export function test_ritual(): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly shadow_integration_ritual: () => any;
  readonly energy_attunement_ritual: () => any;
  readonly void_contemplation_ritual: () => any;
  readonly archetype_invocation_ritual: () => any;
  readonly test_ritual: () => any;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
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
