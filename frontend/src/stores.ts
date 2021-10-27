import {derived, Writable, writable} from "svelte/store";
import {mafiaHost} from "./variables";

export const user: Writable<{ name: any; guid: any }> = writable(null)
export const eventsUrl = derived([user], ([$u]) => `${mafiaHost}/events/${$u.guid}`, "")
export const pendingEvents = writable([]);
