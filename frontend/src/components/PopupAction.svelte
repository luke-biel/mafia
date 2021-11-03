<script lang="ts">
    import {EventKind, EventMsg} from "../dto/event";
    import {createEventDispatcher} from "svelte";
    import backend from "../backend";
    import {Action} from "../dto/action";

    export let event: EventMsg;

    class DisplayEntry {
        internalOption: String;
        displayName: String;
    }

    const capabilities = async (): Promise<Array<DisplayEntry>> => {
        const capabilities = await backend.capabilities(event)
        const gameState = await backend.gameState()

        let entries = []
        capabilities['players'].forEach(cap => {
            const id = cap['id']
            const player = gameState['players'].find((player) => player.id === id);
            entries.push({internalOption: id, displayName: player?.name ?? '(nobody)'})
        })
        return entries
    }

    const select = (id: String) => {
        dispatch('select', Action.from(event, {id}))
    }

    const dispatch = createEventDispatcher();

    let popupTitle = 'Unsupported event kind, call your developer';
    let options: Promise<Array<DisplayEntry>> = capabilities()
    switch (event.msg) {
        case EventKind.CheckGoodBad:
            popupTitle = 'Wybierz czyj pokój przeszukać (katani):';
            break;
        case EventKind.CheckCard:
            popupTitle = 'Wybierz czyje sekrety chcesz poznać (plo):'
            break;
        case EventKind.Heal:
            popupTitle = 'Który gracz odwiedzi Twój gabinet dziś w nocy (lekarz)?'
            break;
        case EventKind.SelectBlackmailed:
            popupTitle = 'Którego gracza chcesz szantażować (szantażysta)?'
            break;
        case EventKind.FinishPatient:
            popupTitle = 'Komu podasz zastrzyk (pawulon)?'
            break;
        case EventKind.MarkForDeath:
            popupTitle = 'Naznacz kogoś na śmierć (anioł)'
            break;
        case EventKind.SelectDiabolized:
            popupTitle = 'Kto zostanie Twoją marionetką po śmierci (diabolistka)?'
            break;
    }
</script>

<div class="mafia-popup-action">
    {popupTitle}
    {#if options}
        {#await options then entries}
            <ul>
                {#each entries as entry}
                    <li>
                        <button on:click={() => select(entry.internalOption)}>{entry.displayName}</button>
                    </li>
                {/each}
            </ul>
        {/await}
    {/if}
</div>

<style>
    .mafia-popup-action {
        top: 0;
        left: 0;
        width: 16em;
        margin: 1em;
        padding: 1em;
        position: absolute;
        border: 1px black solid;
        background-color: white;
    }
</style>
