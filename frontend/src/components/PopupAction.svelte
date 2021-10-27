<script lang="ts">
    import {EventKind, EventMsg} from "../dto/event";

    export let event: EventMsg;

    let popupTitle = 'Unsupported event kind, call your developer';
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

<div>

</div>
