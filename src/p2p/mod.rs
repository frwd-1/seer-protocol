use futures::prelude::*;
use libp2p::{
    core::identity,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmBuilder, SwarmEvent},
    Multiaddr, NetworkBehaviour, PeerId,
};
use std::error::Error;

#[derive(NetworkBehaviour)]
struct Behaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

pub struct P2PNetwork {
    pub peer_id: PeerId,
    pub swarm: Swarm<Behaviour>,
}

impl P2PNetwork {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(local_key.public());

        let floodsub = Floodsub::new(peer_id.clone());
        let mdns = Mdns::new(MdnsConfig::default())?;

        let behaviour = Behaviour { floodsub, mdns };
        let swarm = SwarmBuilder::new(behaviour, local_key.public(), None).build();

        Ok(P2PNetwork { peer_id, swarm })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let topic = Topic::new("seer-protocol");

        self.swarm.behaviour_mut().floodsub.subscribe(topic.clone());

        loop {
            match self.swarm.next().await {
                SwarmEvent::Behaviour(FloodsubEvent::Message(message)) => {
                    println!(
                        "Received: '{:?}' from {:?}",
                        String::from_utf8_lossy(&message.data),
                        message.source
                    );
                }
                SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                    for (peer_id, _addr) in peers {
                        self.swarm
                            .behaviour_mut()
                            .floodsub
                            .add_node_to_partial_view(peer_id);
                    }
                }
                SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => {
                    for (peer_id, _addr) in expired {
                        self.swarm
                            .behaviour_mut()
                            .floodsub
                            .remove_node_from_partial_view(&peer_id);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn broadcast(&mut self, message: String) {
        let topic = Topic::new("seer-protocol");
        self.swarm
            .behaviour_mut()
            .floodsub
            .publish(topic, message.as_bytes());
    }
}
