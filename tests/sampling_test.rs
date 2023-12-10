use libp2p::{
    futures::StreamExt, swarm::dummy::Behaviour as DummyBehaviour, swarm::Swarm, Multiaddr,
    SwarmBuilder,
};

#[test]
fn test_ping() {
    let mut swarm0 = create_swarm();
    let mut swarm1 = create_swarm();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.spawn(async move {
        swarm0
            .listen_on("/ip4/127.0.0.1/tcp/8888".parse().unwrap())
            .unwrap();

        while let Some(event) = swarm0.next().await {
            println!("swarm0: {:?}", event);
        }
    });

    runtime.block_on(async move {
        // wait for 1 event to make sure swarm0 is listening
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        swarm1
            .dial("/ip4/127.0.0.1/tcp/8888".parse::<Multiaddr>().unwrap())
            .unwrap();

        while let Some(event) = swarm1.next().await {
            println!("swarm1: {:?}", event);
        }
    });
}

fn create_swarm() -> Swarm<DummyBehaviour> {
    SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            Default::default(),
            libp2p::tls::Config::new,
            libp2p::yamux::Config::default,
        )
        .unwrap()
        .with_behaviour(|_| DummyBehaviour)
        .unwrap()
        .build()
}
