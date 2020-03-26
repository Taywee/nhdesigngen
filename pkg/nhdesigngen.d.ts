/* tslint:disable */
/* eslint-disable */
/**
*/
export function main(): void;
/**
* @returns {number} 
*/
export function design_new(): number;
/**
* @param {number} design 
* @returns {any} 
*/
export function design_palette(design: number): any;
/**
* @param {number} design 
* @returns {any} 
*/
export function design_dimensions(design: number): any;
/**
* @param {number} design 
* @param {string} ditherer 
* @returns {any} 
*/
export function design_generate(design: number, ditherer: string): any;
/**
* @param {number} design 
* @param {any} buffers 
*/
export function design_load_palette(design: number, buffers: any): void;
/**
* @param {number} design 
* @param {string} optimizer 
*/
export function design_optimize_palette(design: number, optimizer: string): void;
/**
* @param {number} design 
* @param {any} buffer 
* @param {number} width 
* @param {number} height 
*/
export function design_load_image(design: number, buffer: any, width: number, height: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: () => void;
  readonly design_new: () => number;
  readonly design_palette: (a: number) => number;
  readonly design_dimensions: (a: number) => number;
  readonly design_generate: (a: number, b: number, c: number) => number;
  readonly design_load_palette: (a: number, b: number) => void;
  readonly design_optimize_palette: (a: number, b: number, c: number) => void;
  readonly design_load_image: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        