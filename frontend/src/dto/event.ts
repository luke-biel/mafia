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
