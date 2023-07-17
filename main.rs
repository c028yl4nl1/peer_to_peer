use libp2p::{identity, Multiaddr, PeerId};
use futures::prelude::*;
use libp2p::{
    ping::{Ping, PingConfig},
    swarm::{Swarm, SwarmEvent, dialer::Dialer},
    Multiaddr, PeerId, Transport,
};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Vamos fazer um Libp2p em Rust!");

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("ID: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = {
        let dialer = Dialer::new(transport.clone())
            .with_timer(Default::default())
            .with_retry(Default::default());
        Swarm::new(transport, behaviour, local_peer_id, dialer)
    };

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial_addr(remote)?;
        println!("Dialed {}", addr);
    }

    loop {
        match swarm.next().await {
            Some(SwarmEvent::NewListenAddr { address }) => {
                println!("Listening on {:?}", address);
            }
            Some(SwarmEvent::Behaviour(event)) => {
                println!("{:?}", event);
            }
            _ => {}
        }
    }
}
