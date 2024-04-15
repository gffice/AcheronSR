use crate::util;

use super::*;

pub async fn on_get_basic_info_cs_req(
    session: &PlayerSession,
    _body: &GetBasicInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_BASIC_INFO_SC_RSP,
            GetBasicInfoScRsp {
                retcode: 0,
                player_setting_info: Some(PlayerSettingInfo::default()),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_hero_basic_type_info_cs_req(
    session: &PlayerSession,
    _body: &GetHeroBasicTypeInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_HERO_BASIC_TYPE_INFO_SC_RSP,
            GetHeroBasicTypeInfoScRsp {
                retcode: 0,
                gender: Gender::Man.into(),
                cur_basic_type: HeroBasicType::BoyWarrior.into(),
                basic_type_info_list: vec![HeroBasicTypeInfo {
                    basic_type: HeroBasicType::BoyWarrior.into(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await
}

pub async fn on_player_heart_beat_cs_req(
    session: &PlayerSession,
    body: &PlayerHeartBeatCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_HEART_BEAT_SC_RSP,
            PlayerHeartBeatScRsp {
                retcode: 0,
                client_time_ms: body.client_time_ms,
                server_time_ms: util::cur_timestamp_ms(),
                download_data: Some(ClientDownloadData {
                    version: 51,
                    time: util::cur_timestamp_ms() as i64,
                    data: rbase64::decode("G0x1YVMBGZMNChoKBAQICHhWAAAAAAAAAAAAAAAod0ABKEBDOlxVc2Vyc1x4ZW9uZGV2XERvd25sb2Fkc1xhY2hlcm9uLmx1YQAAAAAAAAAAAAEEHwAAACQAQAApQEAAKYBAACnAQABWAAEALIAAAR1AQQCkgEEA5ABAAOnAwQHpAMIB6UDCAawAAAEsgAAAH8BChSQAQAApQEAAKYBAACnAQABWAAMALIAAAR1AQQCkgEEA5ABAAOnAwQHpAMIB6UDCAawAAAEsgAAAH0BDhRkAgAAOAAAABANDUwQMVW5pdHlFbmdpbmUEC0dhbWVPYmplY3QEBUZpbmQEKVVJUm9vdC9BYm92ZURpYWxvZy9CZXRhSGludERpYWxvZyhDbG9uZSkEF0dldENvbXBvbmVudEluQ2hpbGRyZW4EB3R5cGVvZgQEUlBHBAdDbGllbnQEDkxvY2FsaXplZFRleHQEBXRleHQULUFjaGVyb25TUiBpcyBhIGZyZWUgYW5kIG9wZW4gc291cmNlIHNvZnR3YXJlBAxWZXJzaW9uVGV4dBQuVmlzaXQgZGlzY29yZC5nZy9yZXZlcnNlZHJvb21zIGZvciBtb3JlIGluZm8hAQAAAAEAAAAAAB8AAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAACAAAAAgAAAAIAAAACAAAAAgAAAAIAAAACAAAAAgAAAAIAAAACAAAAAgAAAAIAAAACAAAAAgAAAAIAAAACAAAAAAAAAAEAAAAFX0VOVg==").unwrap()
                }),
            },
        )
        .await
}
