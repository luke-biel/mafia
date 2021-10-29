<script lang="ts">
    import backend from "../backend"

    const gameState = backend.gameState()
</script>

<div>
    <ul>
        {#await gameState}
            Fetching lobby...
        {:then lobby}
            {#each lobby.players as user}
                <li>{user.name}</li>
            {/each}
        {/await}
    </ul>
</div>
