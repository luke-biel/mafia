import {mafiaHost} from "./variables";
import {EventKind, EventMsg} from "./dto/event";
import type {Action} from "./dto/action";

class Backend {
    gameState(): Promise<object> {
        return fetch(`${mafiaHost}/game_state`).then((v) => v.json())
    }

    register(name: string): Promise<object> {
        return fetch(`${mafiaHost}/register`, {
            method: 'POST',
            mode: 'cors',
            credentials: 'include',
            cache: 'no-cache',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name,
            })
        }).then((v) => v.json());
    }

    user(guid: string): Promise<object | null> {
        return fetch(`${mafiaHost}/user/${guid}`, {
            method: 'GET',
            mode: 'cors',
            credentials: 'include',
            cache: 'no-cache'
        }).then((v) => {
            if (v.status === 200) {
                return v.json()
            } else {
                return null
            }
        })
    }

    capabilities(event: EventMsg): Promise<object> {
        return fetch(`${mafiaHost}/capabilities`, {
            method: 'POST',
            mode: 'cors',
            credentials: 'include',
            cache: 'no-cache',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({request: EventKind[event.msg]})
        }).then((response) => response.json())
    }

    events() {
        return new EventSource(`${mafiaHost}/events`, {
            withCredentials: true
        });
    }

    action(action: Action): Promise<Response> {
        return fetch(`${mafiaHost}/action`, {
            method: 'POST',
            mode: 'cors',
            credentials: 'include',
            cache: 'no-cache',
            headers: {
                'Content-Type': 'application/json'
            },
            body: action.toJSONString()
        })
    }
}

export default new Backend()
