<script lang="ts">
    import {eventsUrl, user} from "../stores";
    import {EventMsg} from "../dto/event";
    import Summary from "../components/Summary.svelte";
    import History from "../components/History.svelte";
    import EventList from "../components/EventList.svelte";

    enum Tab {
        EVENTS,
        SUMMARY,
        HISTORY,
    }

    let tab: Tab = Tab.SUMMARY
    let evtStream;
    let history = []
    let pendingEvents = []

    $: evtStream = new EventSource($eventsUrl);
    $: evtStream.onmessage = (evt) => {
        const data = new EventMsg(evt.data)
        history = [...history, data]
        pendingEvents = [...pendingEvents, event];
    }

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
    <EventList items={pendingEvents} />
{:else if tab === Tab.HISTORY}
    <p>Historical notifications:</p>
    <History items={history}/>
{/if}

