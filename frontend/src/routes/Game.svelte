<script lang="ts">
    import {user} from "../stores";
    import {EventMsg} from "../dto/event";
    import Summary from "../components/Summary.svelte";
    import History from "../components/History.svelte";
    import EventList from "../components/EventList.svelte";
    import PopupAction from "../components/PopupAction.svelte";
    import {mafiaHost} from "../variables";

    enum Tab {
        EVENTS,
        SUMMARY,
        HISTORY,
    }

    const openPopup = (event: CustomEvent) => {
        popupEvent = event.detail
    }

    const closePopup = (event: CustomEvent) => {

        popupEvent = null
    }

    let evtStream = new EventSource(`${mafiaHost}/events`, {
        withCredentials: true
    });
    evtStream.onmessage = (evt) => {
        const data = new EventMsg(evt.data)
        history = [...history, data]
        if (data.requiresResponse) {
            pendingEvents = [...pendingEvents, data];
        }
    }

    let tab: Tab = Tab.SUMMARY
    let history: Array<EventMsg> = []
    let pendingEvents: Array<EventMsg> = []
    let popupEvent: EventMsg = null;
</script>

<nav>
    <button on:click={() => tab = Tab.SUMMARY}>Summary</button>
    <button on:click={() => tab = Tab.EVENTS}>Events
        {#if pendingEvents.length > 0}!{/if}
    </button>
    <button on:click={() => tab = Tab.HISTORY}>Debug history</button>
</nav>
<span>User: <b>{$user.name}</b></span>
{#if tab === Tab.SUMMARY}
    <p>Lobby:</p>
    <Summary/>
{:else if tab === Tab.EVENTS}
    <p>Events awaiting your action:</p>
    <EventList items={pendingEvents} on:select={openPopup}/>
{:else if tab === Tab.HISTORY}
    <p>Historical notifications:</p>
    <History items={history}/>
{/if}

{#if popupEvent !== null}
    <PopupAction event={popupEvent} on:select={closePopup}/>
{/if}
