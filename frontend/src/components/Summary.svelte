<script lang="ts">
    import {mafiaHost} from "../variables";

    const lobby = fetch(`${mafiaHost}/game_state`).then((v) => v.json())
</script>

<div>
    <ul>
        {#await lobby}
            Fetching lobby...
        {:then lobby}
            {#each lobby.players as user}
                <li>{user.name}</li>
            {/each}
        {/await}
    </ul>
</div>
