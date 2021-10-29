<script lang="ts">
    import {EventKind, EventMsg} from "../dto/event";
    import {mafiaHost} from "../variables";
    import {createEventDispatcher} from "svelte";

    export let event: EventMsg;

    class DisplayEntry {
        internalOption: String;
        displayName: String;
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

    const capabilities = async (): Promise<Array<DisplayEntry>> => {
        const capabilities = await fetch(`${mafiaHost}/capabilities`, {
            method: 'POST',
            mode: 'cors',
            credentials: 'include',
            cache: 'no-cache',
            headers: {
                'Content-Type': 'application/json'
            }, body: JSON.stringify({request: EventKind[event.msg]})
        }).then((response) => response.json())
        const game_state = await fetch(`${mafiaHost}/game_state`).then((response) => response.json())

        let entries = []
        capabilities['players'].forEach((id) => {
            const player = game_state['players'].find((player) => player.id === id);
            entries.push({ internalOption: id, displayName: player?.name ?? '(nobody)' })
        })
        return entries
    }

    const select = (id: String) => {
        dispatch('select', { id })
    }
</script>

<div class="mafia-popup-action">
    {popupTitle}
    {#if options}
        {#await options then entries}
            <ul>
                {#each entries as entry}
                    <button  on:click={() => select(entry.internalOption)}>{entry.displayName}</button>
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
