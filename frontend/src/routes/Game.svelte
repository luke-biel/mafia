<script lang="ts">
    import {eventsUrl, user} from "../stores";
    import Summary from "../components/Summary.svelte";
    import History from "../components/History.svelte";

    enum Tab {
        SUMMARY,
        HISTORY,
    }

    function handleEvent(event) {
        console.log(event)
    }

    let tab: Tab = Tab.SUMMARY
    let evtStream;
    let history = []

    $: evtStream = new EventSource($eventsUrl);
    $: evtStream.onmessage = (evt) => {
        history = [...history, evt.data]
        handleEvent(evt.data)
    }

</script>

<nav>
    <button on:click={() => tab = Tab.SUMMARY}>Summary</button>
    <button on:click={() => tab = Tab.HISTORY}>History</button>
</nav>
<span>User: <b>{$user.name}</b></span>
{#if tab === Tab.SUMMARY}
    <Summary />
{:else if tab === Tab.HISTORY}
    <History items={history} />
{/if}

