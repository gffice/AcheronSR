use super::*;
use crate::game::globals;

pub async fn on_start_cocoon_stage_cs_req(
    session: &PlayerSession,
    body: &StartCocoonStageCsReq,
) -> Result<()> {
    let player_info = session.player_info();

    let rsp = StartCocoonStageScRsp {
        retcode: 0,
        prop_entity_id: body.prop_entity_id,
        cocoon_id: body.cocoon_id,
        wave: body.wave,
        battle_info: Some(SceneBattleInfo {
            stage_id: 201012311,
            logic_random_seed: 4444,
            battle_id: 1,
            battle_avatar_list: player_info
                .lineup
                .avatar_list
                .iter()
                .map(|avatar| BattleAvatar {
                    index: avatar.slot,
                    id: avatar.id,
                    level: 80,
                    promotion: 6,
                    rank: 6,
                    hp: 10000,
                    avatar_type: 3,
                    sp: Some(AmountInfo {
                        cur_amount: 10000,
                        max_amount: 10000,
                    }),
                    ..Default::default()
                })
                .collect(),
            monster_wave_list: globals
                .monster_wave_list
                .iter()
                .map(|monster_list| SceneMonsterWave {
                    monster_list: monster_list
                        .iter()
                        .map(|id| SceneMonsterParam {
                            monster_id: *id,
                            ..Default::default()
                        })
                        .collect(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }),
    };

    session.send(CMD_START_COCOON_STAGE_SC_RSP, rsp).await
}

pub async fn on_pve_battle_result_cs_req(
    session: &PlayerSession,
    body: &PveBattleResultCsReq,
) -> Result<()> {
    session
        .send(
            CMD_P_V_E_BATTLE_RESULT_SC_RSP,
            PveBattleResultScRsp {
                retcode: 0,
                end_status: body.end_status,
                battle_id: body.battle_id,
                ..Default::default()
            },
        )
        .await
}
