import {Writable, writable} from "svelte/store";

export const user: Writable<{ name: any; guid: any }> = writable(null)
export const host = writable("")
