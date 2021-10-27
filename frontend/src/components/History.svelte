<script lang="ts">
    import {EventKind} from "../dto/event";

    export let items;
</script>

<div>
    <ul>
        {#each items as ev}
            <li>{EventKind[ev]}</li>
        {/each}
    </ul>
</div>
