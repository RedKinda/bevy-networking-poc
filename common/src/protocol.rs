use bevy::prelude::*;
use bevy_networking_turbulence::*;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use crate::events::{PlayerId};

pub type NetworkObjectId = u32;

pub fn generate_network_id() -> NetworkObjectId {
    crate::get_random()
}

const GAME_EVENT_CHANNEL_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024 },
    message_buffer_size: 8,
    packet_buffer_size: 8
};

const META_CHANNEL_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024 },
    message_buffer_size: 8,
    packet_buffer_size: 8
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaInformation {
    ClientIdentificationMessage(ClientIdentification),
    DisconnectReason(String)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientIdentification {
    pub player_id: crate::events::PlayerId
}

impl ClientIdentification {
    pub fn new(id: PlayerId) -> Self {
        Self {
            player_id: id
        }
    }

    pub fn update(&mut self, other: Self) {
        self.player_id = other.player_id;
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Component)]
pub struct NetworkSync {
    pub unique_id: NetworkObjectId,
}

impl NetworkSync {
    pub fn new() -> Self {
        NetworkSync {
            unique_id: generate_network_id()
        }
    }
}


/*
impl Display for NetworkSync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(format!("Network sync number {}", self.unique_id).as_str())
    }
}

 */

pub fn network_setup(net: &mut ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<crate::events::GameEvent>(GAME_EVENT_CHANNEL_SETTINGS)
            .unwrap();
        builder
            .register::<MetaInformation>(META_CHANNEL_SETTINGS)
            .unwrap()
    });
}
