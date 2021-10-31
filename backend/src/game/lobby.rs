use crate::comms::{
    Broadcast, Context, Details, MessageInBody, MessageOut, Meta, Notification, ResponseKind,
};
use crate::game::card::syndicate::SYNDICATE_BLANK;
use crate::game::card::{Faction, Role, Value};
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Lobby {
    pub roles: HashMap<Uuid, Function>,
    pub time_of_day: TimeOfDay,
    pub day: usize,
    pub modifiers: GameModifiers,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub card: &'static (dyn Role + Send + Sync),
    pub alive: bool,
    pub modifiers: RoleModifiers,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum TimeOfDay {
    Day,
    Dusk,
    Night,
}

#[derive(Clone, Debug, Default)]
pub struct GameModifiers {
    pub is_gun_shop_dead_during_day: bool,
}

#[derive(Clone, Debug, Default)]
pub struct RoleModifiers {
    pub diabolised: bool,
    pub marked_by_aod: bool,
    pub blackmailed_by: Option<Uuid>,
    pub blackmails: Option<Uuid>,
}

impl const Default for TimeOfDay {
    fn default() -> Self {
        Self::Night
    }
}

impl TimeOfDay {
    pub fn advance(&mut self) -> usize {
        match self {
            TimeOfDay::Day => {
                *self = TimeOfDay::Dusk;
                0
            }
            TimeOfDay::Dusk => {
                *self = TimeOfDay::Night;
                0
            }
            TimeOfDay::Night => {
                *self = TimeOfDay::Day;
                1
            }
        }
    }
}

impl Lobby {
    fn check_victory(&self) -> Option<Faction> {
        for faction in [Faction::City, Faction::Mafia, Faction::Syndicate] {
            if self
                .roles
                .values()
                .filter(|player| player.alive)
                .all(|player| player.card.faction() == faction)
            {
                return Some(faction);
            }
        }
        None
    }

    pub fn update(&mut self, deltas: HashMap<Meta, MessageInBody>) -> Vec<(Uuid, MessageOut)> {
        if let Some(faction) = self.check_victory() {
            return self
                .roles
                .keys()
                .map(|id| {
                    (
                        *id,
                        MessageOut {
                            requires_response: false,
                            msg: Context::Broadcast(Broadcast::GameEnd),
                            details: Some(Details::Faction(faction)),
                        },
                    )
                })
                .collect::<Vec<_>>();
        }

        let mut killed = Self::resolve_mafia_kill_and_heal(&deltas);
        killed.extend(self.resolve_aod(&deltas));
        killed.extend(self.resolve_death_of_blackmailer(killed.clone().into_iter()));

        let mut responses = Vec::new();
        for id in killed {
            let function = self.roles.get_mut(&id).expect("killed player should exist");
            if function.modifiers.diabolised {
                responses.push((
                    id,
                    MessageOut {
                        requires_response: false,
                        msg: Context::Notification(Notification::RaisedFromGrave),
                        details: None,
                    },
                ));
                function.card = &SYNDICATE_BLANK;
            } else {
                responses.push((
                    id,
                    MessageOut {
                        requires_response: false,
                        msg: Context::Notification(Notification::Killed),
                        details: None,
                    },
                ));
                function.alive = false;
            }
        }

        responses.extend(self.resolve_blackmailing(&deltas));
        responses.extend(self.resolve_checking(&deltas));

        self.day += self.time_of_day.advance();

        responses
    }

    fn resolve_mafia_kill_and_heal(deltas: &HashMap<Meta, MessageInBody>) -> Vec<Uuid> {
        let pavulon_targets = deltas
            .iter()
            .filter(|(meta, _)| meta.response_kind == ResponseKind::FinishTarget)
            .collect::<Vec<_>>();
        let real_heals = deltas
            .iter()
            .filter(|(meta, _)| meta.response_kind == ResponseKind::HealTarget)
            .filter(|(_, h_body)| {
                !pavulon_targets
                    .iter()
                    .any(|(_, p_body)| h_body.id() == p_body.id())
            })
            .collect::<Vec<_>>();

        deltas
            .iter()
            .filter(|(meta, _)| meta.response_kind == ResponseKind::ShootTarget)
            .filter(|(_, k_body)| {
                !real_heals
                    .iter()
                    .any(|(_, h_body)| k_body.id() == h_body.id())
            })
            .map(|(_, body)| body.id())
            .collect()
    }

    fn resolve_aod(&mut self, deltas: &HashMap<Meta, MessageInBody>) -> impl Iterator<Item = Uuid> {
        let mut killed = Vec::new();
        for (_, body) in deltas
            .iter()
            .filter(|(meta, _)| meta.response_kind == ResponseKind::DeathMarkTarget)
        {
            let id = body.id();

            let player = self
                .roles
                .get_mut(&id)
                .expect("player marked by aod is missing");
            if !player.modifiers.marked_by_aod {
                player.modifiers.marked_by_aod = true;
            } else {
                killed.push(id)
            }
        }
        killed.into_iter()
    }

    fn resolve_death_of_blackmailer(
        &self,
        killed: impl Iterator<Item = Uuid>,
    ) -> impl Iterator<Item = Uuid> {
        let mut res = Vec::new();
        for id in killed {
            let player = self.roles.get(&id).expect("player is missing");
            if player.card.faction() == Faction::Mafia && player.card.value() == Value::King {
                res.push(
                    player
                        .modifiers
                        .blackmails
                        .expect("blackmailer didn't blackmail anybody"),
                )
            }
        }
        res.into_iter()
    }

    fn resolve_blackmailing(
        &mut self,
        deltas: &HashMap<Meta, MessageInBody>,
    ) -> impl Iterator<Item = (Uuid, MessageOut)> {
        let mut responses = Vec::new();
        for (meta, body) in deltas
            .iter()
            .filter(|(meta, _)| meta.response_kind == ResponseKind::BlackmailTarget)
        {
            self.roles
                .get_mut(&body.id())
                .expect("blackmailed missing")
                .modifiers
                .blackmailed_by = Some(meta.guid);
            self.roles
                .get_mut(&meta.guid)
                .expect("blackmailer missing")
                .modifiers
                .blackmails = Some(body.id());

            responses.push((
                body.id(),
                MessageOut {
                    requires_response: false,
                    msg: Context::Notification(Notification::Blackmailed),
                    details: Some(Details::Id(meta.guid)),
                },
            ))
        }
        responses.into_iter()
    }

    fn resolve_checking(
        &self,
        deltas: &HashMap<Meta, MessageInBody>,
    ) -> impl Iterator<Item = (Uuid, MessageOut)> {
        let mut responses = Vec::new();

        for (meta, body) in deltas
            .iter()
            .filter(|(meta, _)| meta.response_kind == ResponseKind::CheckCardTarget)
        {
            let function = self.roles.get(&body.id()).expect("checked player");
            responses.push((
                meta.guid,
                MessageOut {
                    requires_response: false,
                    msg: Context::Notification(Notification::CardCheck),
                    details: Some(Details::Card(format!("{:?}", function.card))),
                },
            ))
        }

        for (meta, body) in deltas
            .iter()
            .filter(|(meta, _)| meta.response_kind == ResponseKind::CheckGoodBadTarget)
        {
            let function = self.roles.get(&body.id()).expect("checked player");
            responses.push((
                meta.guid,
                MessageOut {
                    requires_response: false,
                    msg: Context::Notification(Notification::CardCheck),
                    details: Some(Details::Card(
                        (if function.card.faction() == Faction::City
                            || (function.card.faction() == Faction::Mafia
                                && function.card.value() == Value::Queen)
                        {
                            "good"
                        } else {
                            "bad"
                        })
                        .to_string(), // TODO: This is crap
                    )),
                },
            ))
        }

        responses.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::comms::{MessageInBody, Meta, ResponseKind};
    use crate::game::lobby::Lobby;
    use std::collections::HashMap;
    use test_case::test_case;
    use uuid::Uuid;

    #[test_case(
        &[],
        &[],
        &[]
        => Vec::<String>::new(); "No input = no output"
    )]
    #[test_case(
        &["7cb32d55-8568-4494-86ad-09dac1748001"],
        &[],
        &[]
        => &["7cb32d55-8568-4494-86ad-09dac1748001"][..]; "One kill")]
    #[test_case(
        &["7cb32d55-8568-4494-86ad-09dac1748001"],
        &["7cb32d55-8568-4494-86ad-09dac1748001"],
        &[]
        => Vec::<String>::new(); "One kill, one heal"
    )]
    #[test_case(
        &["7cb32d55-8568-4494-86ad-09dac1748001"],
        &[],
        &["7cb32d55-8568-4494-86ad-09dac1748001"]
        => vec!["7cb32d55-8568-4494-86ad-09dac1748001"]; "One kill, one finish"
    )]
    #[test_case(
        &["7cb32d55-8568-4494-86ad-09dac1748001"],
        &["7cb32d55-8568-4494-86ad-09dac1748001"],
        &["7cb32d55-8568-4494-86ad-09dac1748001"]
        => vec!["7cb32d55-8568-4494-86ad-09dac1748001"]; "One of each"
    )]
    #[test_case(
        &["7cb32d55-8568-4494-86ad-09dac1748001", "55e4001d-55be-49e3-ab33-58219a1cf061"],
        &["7cb32d55-8568-4494-86ad-09dac1748001"],
        &[]
        => vec!["55e4001d-55be-49e3-ab33-58219a1cf061"]; "Two kill, one heal"
    )]
    #[test_case(
        &["7cb32d55-8568-4494-86ad-09dac1748001", "55e4001d-55be-49e3-ab33-58219a1cf061"],
        &[],
        &[]
        => vec!["55e4001d-55be-49e3-ab33-58219a1cf061", "7cb32d55-8568-4494-86ad-09dac1748001"]; "Two kill"
    )]
    #[test_case(
        &["7cb32d55-8568-4494-86ad-09dac1748001", "55e4001d-55be-49e3-ab33-58219a1cf061"],
        &["7cb32d55-8568-4494-86ad-09dac1748001"],
        &["7cb32d55-8568-4494-86ad-09dac1748001"]
        => vec!["55e4001d-55be-49e3-ab33-58219a1cf061", "7cb32d55-8568-4494-86ad-09dac1748001"]; "Two kill, one heal, one finish"
    )]
    #[test_case(
        &["7cb32d55-8568-4494-86ad-09dac1748001", "55e4001d-55be-49e3-ab33-58219a1cf061"],
        &["7cb32d55-8568-4494-86ad-09dac1748001"],
        &["55e4001d-55be-49e3-ab33-58219a1cf061"]
        => vec!["55e4001d-55be-49e3-ab33-58219a1cf061"]; "Two kill, one heal, mismatch finish"
    )]
    fn resolves_mafia_and_doctor_night_activity(
        kills: &[&str],
        heals: &[&str],
        finishes: &[&str],
    ) -> Vec<String> {
        let mut deltas = HashMap::new();

        for id in kills {
            deltas.insert(
                Meta {
                    guid: Uuid::new_v4(),
                    response_kind: ResponseKind::ShootTarget,
                },
                MessageInBody::Id(id.parse().expect("parse uuid kill")),
            );
        }
        for id in heals {
            deltas.insert(
                Meta {
                    guid: Uuid::new_v4(),
                    response_kind: ResponseKind::HealTarget,
                },
                MessageInBody::Id(id.parse().expect("parse uuid heal")),
            );
        }
        for id in finishes {
            deltas.insert(
                Meta {
                    guid: Uuid::new_v4(),
                    response_kind: ResponseKind::FinishTarget,
                },
                MessageInBody::Id(id.parse().expect("parse uuid finishes")),
            );
        }

        let mut s: Vec<_> = Lobby::resolve_mafia_kill_and_heal(&deltas)
            .iter()
            .map(|(_, body)| body.id().to_string())
            .collect();
        s.sort();
        s
    }
}
