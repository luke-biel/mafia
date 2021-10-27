<script lang="ts">
    export let items;
</script>

<div>
    <p>Historical notifications:</p>
    <ul>
    {#each items as ev}
        <li>{ev}</li>
    {/each}
    </ul>
</div>
