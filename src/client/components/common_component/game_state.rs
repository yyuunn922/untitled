use bevy::prelude::{Component, Resource, States};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

// 레벨 상태
#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum LevelState {
    MainMenu,
    MainVillage,
    Field,
}

// UI 상태
#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum UiState {
    None,
    MainMenu,
}

#[derive(Resource)]
pub struct Network {
    pub tcp_sender: UnboundedSender<String>,
    pub tcp_receiver: UnboundedReceiver<String>,
    pub udp_sender: UnboundedSender<String>,
    pub udp_receiver: UnboundedReceiver<String>,
}

impl Default for Network {
    fn default() -> Self {
        // 네트워크 수신 및 발신기 생성
        let (tcp_sender, tcp_receiver) = tokio::sync::mpsc::unbounded_channel();
        let (udp_sender, udp_receiver) = tokio::sync::mpsc::unbounded_channel();

        Self {
            tcp_sender,
            tcp_receiver,
            udp_sender,
            udp_receiver,
        }
    }
}