<script lang="ts">
    import {EventMsg} from "../dto/event";
    import {createEventDispatcher} from "svelte";

    export let items: Array<EventMsg>;

    const dispatch = createEventDispatcher();
</script>

<ul>
    {#each items as item}
        <li>
            <button on:click={() => dispatch('select', item)}>{item.actionTitle() ?? '---'} ({item.role()})</button>
        </li>
    {/each}
</ul>
