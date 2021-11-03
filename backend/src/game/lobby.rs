use crate::comms::incoming::Meta;
use crate::comms::incoming::{ActionResponse, VoteKind};
use crate::comms::outgoing::{Broadcast, MessageOut, Notification};
use crate::game::card::{Faction, Role, Value};
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Lobby {
    pub roles: HashMap<Uuid, Function>,
    pub time_of_day: TimeOfDay,
    pub day: usize,
    pub modifiers: GameModifiers,
    pub vote_proposals: Vec<(Uuid, VoteKind)>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub card: Role,
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

    pub fn update(&mut self, deltas: HashMap<Meta, ActionResponse>) -> Vec<(Uuid, MessageOut)> {
        let mut killed = Self::resolve_mafia_kill_and_heal(&deltas);
        killed.extend(self.resolve_aod(&deltas));
        killed.extend(self.resolve_death_of_blackmailer(killed.clone().into_iter()));

        let mut responses = Vec::new();
        for id in killed {
            let function = self.roles.get_mut(&id).expect("killed player should exist");
            if function.modifiers.diabolised {
                responses.push((id, Notification::raised_from_grave()));
                function.card = Role::SyndicateBlank;
            } else {
                responses.push((id, Notification::killed()));
                function.alive = false;
            }
        }

        responses.extend(self.resolve_blackmailing(&deltas));
        responses.extend(self.resolve_checking(&deltas));

        self.resolve_vote_proposals(&deltas);

        self.day += self.time_of_day.advance();

        for (id, _) in self.roles.iter().filter(|(_, fun)| fun.alive) {
            responses.push((*id, Broadcast::time_passes(self.day, self.time_of_day)))
        }

        if let Some(faction) = self.check_victory() {
            let mut all = self
                .roles
                .keys()
                .map(|id| (*id, Broadcast::game_end(faction)))
                .collect::<Vec<_>>();
            all.extend(responses.into_iter());
            all
        } else {
            responses
        }
    }

    fn resolve_vote_proposals(&mut self, deltas: &HashMap<Meta, ActionResponse>) {
        let vote_proposals: Vec<_> = deltas
            .iter()
            .filter_map(|(_, val)| match val {
                ActionResponse::VoteProposal { id, vote_kind } => Some((*id, *vote_kind)),
                _ => None,
            })
            .unique()
            .collect();
        self.vote_proposals = vote_proposals;
    }

    fn resolve_mafia_kill_and_heal(deltas: &HashMap<Meta, ActionResponse>) -> Vec<Uuid> {
        let pavulon_targets = deltas
            .iter()
            .filter(|(_, val)| matches!(val, ActionResponse::FinishTarget { .. }))
            .collect::<Vec<_>>();
        let real_heals = deltas
            .iter()
            .filter(|(_, val)| matches!(val, ActionResponse::HealTarget { .. }))
            .filter(|(_, h_body)| {
                !pavulon_targets
                    .iter()
                    .any(|(_, p_body)| h_body.target_id() == p_body.target_id())
            })
            .collect::<Vec<_>>();

        deltas
            .iter()
            .filter(|(_, val)| matches!(val, ActionResponse::ShootTarget { .. }))
            .filter(|(_, k_body)| {
                !real_heals
                    .iter()
                    .any(|(_, h_body)| k_body.target_id() == h_body.target_id())
            })
            .map(|(_, body)| body.target_id())
            .collect()
    }

    fn resolve_aod(
        &mut self,
        deltas: &HashMap<Meta, ActionResponse>,
    ) -> impl Iterator<Item = Uuid> {
        let mut killed = Vec::new();
        for (_, body) in deltas
            .iter()
            .filter(|(_, val)| matches!(val, ActionResponse::DeathMarkTarget { .. }))
        {
            let id = body.target_id();

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
        deltas: &HashMap<Meta, ActionResponse>,
    ) -> impl Iterator<Item = (Uuid, MessageOut)> {
        let mut responses = Vec::new();
        for (meta, body) in deltas
            .iter()
            .filter(|(_, val)| matches!(val, ActionResponse::BlackmailTarget { .. }))
        {
            self.roles
                .get_mut(&body.target_id())
                .expect("blackmailed missing")
                .modifiers
                .blackmailed_by = Some(meta.guid);
            self.roles
                .get_mut(&meta.guid)
                .expect("blackmailer missing")
                .modifiers
                .blackmails = Some(body.target_id());

            responses.push((body.target_id(), Notification::blackmailed(meta.guid)))
        }
        responses.into_iter()
    }

    fn resolve_checking(
        &self,
        deltas: &HashMap<Meta, ActionResponse>,
    ) -> impl Iterator<Item = (Uuid, MessageOut)> {
        let mut responses = Vec::new();

        for (meta, body) in deltas
            .iter()
            .filter(|(_, val)| matches!(val, ActionResponse::CheckCardTarget { .. }))
        {
            let function = self
                .roles
                .get(&body.target_id())
                .expect("checked player card");
            responses.push((meta.guid, Notification::card_check(function.card)))
        }

        for (meta, body) in deltas
            .iter()
            .filter(|(_, val)| matches!(val, ActionResponse::CheckGoodBadTarget { .. }))
        {
            let function = self
                .roles
                .get(&body.target_id())
                .expect("checked player good_bad");
            responses.push((
                meta.guid,
                Notification::faction_check(
                    function.card.faction() == Faction::City
                        || (function.card.faction() == Faction::Mafia
                            && function.card.value() == Value::Queen),
                ),
            ))
        }

        responses.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::comms::incoming::ActionResponse;
    use crate::comms::incoming::Meta;
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
                },
                ActionResponse::ShootTarget {
                    id: id.parse().expect("parse uuid kill"),
                },
            );
        }
        for id in heals {
            deltas.insert(
                Meta {
                    guid: Uuid::new_v4(),
                },
                ActionResponse::HealTarget {
                    id: id.parse().expect("parse uuid heal"),
                },
            );
        }
        for id in finishes {
            deltas.insert(
                Meta {
                    guid: Uuid::new_v4(),
                },
                ActionResponse::FinishTarget {
                    id: id.parse().expect("parse uuid finishes"),
                },
            );
        }

        let mut s: Vec<_> = Lobby::resolve_mafia_kill_and_heal(&deltas)
            .iter()
            .map(|id| id.to_string())
            .collect();
        s.sort();
        s
    }
}
