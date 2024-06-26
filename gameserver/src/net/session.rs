use anyhow::Result;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use prost::Message;
use std::sync::Arc;
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::{Mutex, MutexGuard},
};

use crate::game::PlayerInfo;

use super::{packet::CommandHandler, NetPacket};

pub struct PlayerSession {
    client_socket: Arc<Mutex<TcpStream>>,
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
}

impl PlayerSession {
    pub fn new(client_socket: TcpStream) -> Self {
        Self {
            client_socket: Arc::new(Mutex::new(client_socket)),
            player_info: Arc::new(AtomicRefCell::new(PlayerInfo::new())),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let net_packet = NetPacket::read(&mut *self.client_socket().await).await?;
            Self::on_message(self, net_packet.cmd_type, net_packet.body).await?;
        }
    }

    pub async fn send(&self, cmd_type: u16, body: impl Message) -> Result<()> {
        let mut buf = Vec::new();
        body.encode(&mut buf)?;

        let payload: Vec<u8> = NetPacket {
            cmd_type,
            head: Vec::new(),
            body: buf,
        }
        .into();

        self.client_socket().await.write_all(&payload).await?;
        Ok(())
    }

    pub async fn client_socket(&self) -> MutexGuard<'_, TcpStream> {
        self.client_socket.lock().await
    }

    pub fn player_uid(&self) -> u32 {
        self.player_info().uid
    }

    pub fn player_info(&self) -> AtomicRef<PlayerInfo> {
        self.player_info.borrow()
    }

    pub fn player_info_mut(&self) -> AtomicRefMut<PlayerInfo> {
        self.player_info.borrow_mut()
    }
}

// Auto implemented
impl CommandHandler for PlayerSession {}
