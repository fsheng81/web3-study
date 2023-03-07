// in example folder. 
// cargo build --example 01-peer
// cargo run --example 01-peer

use libp2p::{identity, PeerId, Multiaddr};
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{Swarm, SwarmEvent, dial_opts::DialOpts};
use futures::prelude::*;
use std::error::Error;

/* 异步使用 async_std 而不是 tokio */
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    /* 1. 生成一个独特的ID */
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("local peer id: {:?}", local_peer_id);

    /* 2. 增加transport 部分，传输层包括了 tcp/udp/socket/ip && 身份认证/加密等 */
    let transport = libp2p::development_transport(local_key).await?;

    /* 3. networkBehaviour 定义上层怎么发消息 */
    /* 以 ping 协议为例 */
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    /* 4. swarm 基本是最重要的部分，把 transport 和 networkBehaviour 结合 */
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    /* 5. 设置监听/通知 如果是非本机的节点呢？ 其他Linux节点 例如三个虚拟机 */
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?);

    if let Some(addr) = std::env::args().nth(1) { /* 怎么使用？ */
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("dial: {}", addr);
    }

    /* 不断轮训 swarm，获取网络事件 */
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("listening on {:?}", address)
            },
            SwarmEvent::Behaviour(event) => println!("event: {:?}", event),
            _ => {}
        }
    }
}