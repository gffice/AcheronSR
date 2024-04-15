use crate::config::*;
use axum::extract::{Path, Query};
use prost::Message;
use proto::{Dispatch, Gateserver, RegionInfo};
use serde::Deserialize;

pub const QUERY_DISPATCH_PATH: &str = "/query_dispatch";
pub const QUERY_GATEWAY_PATH: &str = "/query_gateway/:region_name";

#[tracing::instrument]
pub async fn query_dispatch() -> String {
    let rsp = Dispatch {
        retcode: 0,
        region_list: CONFIGURATION
            .game_servers
            .values()
            .map(|c| RegionInfo {
                name: c.name.clone(),
                title: c.title.clone(),
                env_type: c.env_type.clone(),
                dispatch_url: c.dispatch_url.clone(),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    };

    let mut buff = Vec::new();
    rsp.encode(&mut buff).unwrap();

    rbase64::encode(&buff)
}

#[derive(Deserialize, Debug)]
pub struct QueryGatewayParameters {
    pub version: String,
}

#[tracing::instrument]
pub async fn query_gateway(
    Path(region_name): Path<String>,
    parameters: Query<QueryGatewayParameters>,
) -> String {
    let rsp = if let Some(server_config) = CONFIGURATION.game_servers.get(&region_name) {
        if let Some(version_config) = CONFIGURATION.versions.get(&parameters.version) {
            Gateserver {
                retcode: 0,
                ip: server_config.gateserver_ip.clone(),
                port: u32::from(server_config.gateserver_port),
                asset_bundle_url: version_config.asset_bundle_url.clone(),
                ex_resource_url: version_config.ex_resource_url.clone(),
                lua_url: version_config.lua_url.clone(),
                lua_version: version_config.lua_version.clone(),
                ifix_version: String::from("0"),
                jblkncaoiao: true,
                hjdjakjkdbi: true,
                ldknmcpffim: true,
                feehapamfci: true,
                eebfeohfpph: true,
                dfmjjcfhfea: true,
                najikcgjgan: true,
                giddjofkndm: true,
                use_tcp: server_config.gateserver_protocol == GatewayProtocolType::Tcp,
                ..Default::default()
            }
        } else {
            Gateserver {
                retcode: 9,
                msg: format!("forbidden version: {} or invalid bind", parameters.version),
                ..Default::default()
            }
        }
    } else {
        Gateserver {
            retcode: 9,
            msg: format!("server config for {region_name} not found"),
            ..Default::default()
        }
    };

    let mut buff = Vec::new();
    rsp.encode(&mut buff).unwrap();

    rbase64::encode(&buff)
}
