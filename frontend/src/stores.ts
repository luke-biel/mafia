import {derived, Writable, writable} from "svelte/store";

export const user: Writable<{ name: any; guid: any }> = writable(null)
export const host = writable("")
export const events_url = derived([host, user], ([$h, $u]) => `${$h}/events/${$u.guid}`, "")
