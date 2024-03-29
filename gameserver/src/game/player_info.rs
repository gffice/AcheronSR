use crate::net::PlayerSession;

use super::globals;
use anyhow::Result;
use proto::*;

pub struct PlayerInfo {
    pub lineup: LineupInfo,
}

impl PlayerInfo {
    pub fn new() -> Self {
        Self {
            lineup: default_lineup(),
        }
    }

    pub async fn sync_lineup(&self, session: &PlayerSession) -> Result<()> {
        session
            .send(
                CMD_SYNC_LINEUP_NOTIFY,
                SyncLineupNotify {
                    lineup: Some(self.lineup.clone()),
                    ..Default::default()
                },
            )
            .await
    }
}

fn default_lineup() -> LineupInfo {
    LineupInfo {
        plane_id: 10001,
        name: String::from("Lineup 1"),
        index: 0,
        leader_slot: 0,
        mp: 5,
        mp_max: 5,
        avatar_list: globals
            .lineup
            .iter()
            .enumerate()
            .map(|(idx, id)| LineupAvatar {
                id: *id,
                slot: idx as u32,
                hp: 10000,
                sp: Some(AmountInfo {
                    cur_amount: 10000,
                    max_amount: 10000,
                }),
                satiety: 100,
                avatar_type: 3,
            })
            .collect(),
        ..Default::default()
    }
}
