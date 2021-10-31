<script lang="ts">
    import {actionRequestRole, actionRequestTitle, EventMsg} from "../dto/event";
    import {createEventDispatcher} from "svelte";

    export let items: Array<EventMsg>;

    const dispatch = createEventDispatcher();
</script>

<ul>
    {#each items as item}
        <li>
            <button on:click={() => dispatch('select', item)}>{actionRequestTitle(item.msg) ?? '---'}
                ({actionRequestRole(item.msg)})
            </button>
        </li>
    {/each}
</ul>
