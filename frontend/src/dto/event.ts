// This is mapped to MessageOut variants on backend
export class EventMsg {
    msg: EventKind;

    constructor(jsonStr: string) {
        let jsonObj: any = JSON.parse(jsonStr);
        this['msg'] = EventKind[jsonObj.msg as keyof typeof EventKind]
    }

    isResponseRequested(): boolean {
        switch (this.msg) {
            case EventKind.CheckGoodBad:
                return true;
            case EventKind.CheckCard:
                return true;
            case EventKind.Heal:
                return true;
            case EventKind.SelectBlackmailed:
                return true;
            case EventKind.FinishPatient:
                return true;
            case EventKind.MarkForDeath:
                return true;
            case EventKind.SelectDiabolized:
                return true;
            case EventKind.Shoot:
                return true;
            case EventKind.ProposeVote:
                return true;
            case EventKind.CastVote:
                return true;
            case EventKind.GameStart:
                return false;
            default:
                return false;
        }
    }

    actionTitle(): string | null {
        switch (this.msg) {
            case EventKind.CheckGoodBad:
                return 'sprawdź frakcję';
            case EventKind.CheckCard:
                return 'sprawdź kartę';
            case EventKind.Heal:
                return 'ulecz';
            case EventKind.SelectBlackmailed:
                return 'szantażuj';
            case EventKind.FinishPatient:
                return 'dobij';
            case EventKind.MarkForDeath:
                return 'naznacz';
            case EventKind.SelectDiabolized:
                return 'diabolizuj';
            case EventKind.Shoot:
                return 'strzel';
            case EventKind.ProposeVote:
                return 'zaproponuj głosowanie';
            case EventKind.CastVote:
                return 'oddaj głos';
            case EventKind.GameStart:
                return 'start gry';

        }
    }

    role(): string {
        switch (this.msg) {
            case EventKind.CheckGoodBad:
                return 'katani';
            case EventKind.CheckCard:
                return 'plo';
            case EventKind.Heal:
                return 'lekarz';
            case EventKind.SelectBlackmailed:
                return 'szantażysta';
            case EventKind.FinishPatient:
                return 'pawulon';
            case EventKind.MarkForDeath:
                return 'anioł śmierci';
            case EventKind.SelectDiabolized:
                return 'diabolistka';
            case EventKind.Shoot:
                return 'mafia';
            case EventKind.ProposeVote:
                return 'ogólne';
            case EventKind.CastVote:
                return 'ogólne';
            case EventKind.GameStart:
                return 'ogólne';

        }
    }

    toString(): string {
        return `${this.msg}`
    }
}

export enum EventKind {
    // Action requests
    CheckGoodBad,
    CheckCard,
    Heal,
    SelectBlackmailed,
    FinishPatient,
    MarkForDeath,
    SelectDiabolized,
    Shoot,
    ProposeVote,
    CastVote,

    // Broadcast events
    GameStart
}
