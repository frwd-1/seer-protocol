// use futures::StreamExt;
// use libp2p::{
//     floodsub::{Floodsub, FloodsubEvent, Topic},
//     identity,
//     swarm::{NetworkBehaviour, Swarm, SwarmEvent},
//     PeerId,
// };
// use libp2p_mdns::{tokio::Behaviour as MdnsBehaviour, Config, Event};
// use std::error::Error;
// use std::sync::Arc;
// use tokio::task;

// #[derive(NetworkBehaviour)]
// struct CustomNetworkBehaviour {
//     floodsub: Floodsub,
//     mdns: MdnsBehaviour,
//     // Include any other behaviors or handlers you need
// }

// impl CustomNetworkBehaviour {
//     fn new(peer_id: PeerId) -> Result<Self, Box<dyn Error>> {
//         let floodsub = Floodsub::new(peer_id.clone());
//         let mdns = MdnsBehaviour::new(Config::default())?;

//         Ok(CustomNetworkBehaviour { floodsub, mdns })
//     }
// }

// pub struct P2PNetwork {
//     pub peer_id: PeerId,
//     pub swarm: Arc<Swarm<CustomNetworkBehaviour>>,
// }

// impl P2PNetwork {
//     pub fn new() -> Result<Self, Box<dyn Error>> {
//         let local_key = identity::Keypair::generate_ed25519();
//         let peer_id = PeerId::from(local_key.public());

//         let behaviour = CustomNetworkBehaviour::new(peer_id.clone())?;
//         let transport = libp2p::development_transport(local_key).await?;
//         let swarm = Arc::new(Swarm::new(transport, behaviour, peer_id.clone()));

//         Ok(P2PNetwork { peer_id, swarm })
//     }

//     pub async fn run(&self) -> Result<(), Box<dyn Error>> {
//         let topic = Topic::new("seer-protocol");

//         self.swarm.behaviour_mut().floodsub.subscribe(topic.clone());

//         let swarm = Arc::clone(&self.swarm);
//         task::spawn(async move {
//             loop {
//                 if let Some(event) = swarm.next().await {
//                     match event {
//                         SwarmEvent::Behaviour(FloodsubEvent::Message(message)) => {
//                             println!(
//                                 "Received: '{:?}' from {:?}",
//                                 String::from_utf8_lossy(&message.data),
//                                 message.source
//                             );
//                         }
//                         SwarmEvent::Behaviour(Event::Discovered(peers)) => {
//                             for (peer_id, _addr) in peers {
//                                 swarm
//                                     .behaviour_mut()
//                                     .floodsub
//                                     .add_node_to_partial_view(peer_id);
//                             }
//                         }
//                         SwarmEvent::Behaviour(Event::Expired(expired)) => {
//                             for (peer_id, _addr) in expired {
//                                 swarm
//                                     .behaviour_mut()
//                                     .floodsub
//                                     .remove_node_from_partial_view(&peer_id);
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//         });

//         Ok(())
//     }

//     pub fn broadcast(&self, message: String) {
//         let topic = Topic::new("seer-protocol");
//         self.swarm
//             .behaviour_mut()
//             .floodsub
//             .publish(topic, message.as_bytes());
//     }
// }
