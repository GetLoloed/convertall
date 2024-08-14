import { writable } from 'svelte/store';

export const filesStore = writable([]);
export const selectedFormatStore = writable('png');