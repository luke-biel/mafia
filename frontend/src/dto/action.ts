import {EventKind, EventMsg} from "./event";

export class Action {
    kind: ResponseKind;
    body: object;

    static from(event: EventMsg, body: object): Action {
        let action = new Action()

        action.body = body;

        switch (event.msg) {
            case EventKind.CheckGoodBad:
                action.kind = ResponseKind.CheckGoodBadTarget;
                break;
            case EventKind.CheckCard:
                action.kind = ResponseKind.CheckCardTarget;
                break;
            case EventKind.Heal:
                action.kind = ResponseKind.HealTarget;
                break;
            case EventKind.SelectBlackmailed:
                action.kind = ResponseKind.BlackmailTarget;
                break;
            case EventKind.FinishPatient:
                action.kind = ResponseKind.FinishTarget;
                break;
            case EventKind.MarkForDeath:
                action.kind = ResponseKind.DeathMarkTarget;
                break;
            case EventKind.SelectDiabolized:
                action.kind = ResponseKind.DiabolizationTarget;
                break;
            case EventKind.Shoot:
                action.kind = ResponseKind.ShootTarget;
                break;
            case EventKind.ProposeVote:
                action.kind = ResponseKind.VoteProposal;
                break;
            case EventKind.CastVote:
                action.kind = ResponseKind.VoteTarget;
                break;
        }

        return action
    }

    toJSONString(): string {
        return JSON.stringify({ kind: ResponseKind[this.kind], body: this.body })
    }
}

export enum ResponseKind {
    CheckGoodBadTarget,
    CheckCardTarget,
    HealTarget,
    BlackmailTarget,
    FinishTarget,
    DeathMarkTarget,
    DiabolizationTarget,
    ShootTarget,
    VoteProposal,
    VoteTarget,
}
