<script>
    import {events_url, user} from "./stores";

    $: evtStream = new EventSource($events_url);
    $: evtStream.onmessage = (evt) => {
        events = [...events, evt.data]
    }

    let events = []


</script>

<span>User: <b>{$user.name}</b></span>

{#each events as ev}
    <p>{ev}</p>
{/each}
