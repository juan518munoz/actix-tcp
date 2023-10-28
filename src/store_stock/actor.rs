use actix::prelude::*;

use crate::{
    local::actor::{LocalActor, SetStoreStockAddr as LocalSetStoreStockAddr},
    network::actor::{
        Buy as NetworkBuy, NetworkActor, SetStoreStockAddr as NetworkSetStoreStockAddr,
    },
};

pub struct StoreStockActor {
    network_addr: Option<Addr<NetworkActor>>,
    local_addr: Option<Addr<LocalActor>>,
}

impl Actor for StoreStockActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("StoreStockActor is alive");
    }
}

impl StoreStockActor {
    pub fn new() -> Addr<StoreStockActor> {
        StoreStockActor {
            network_addr: None,
            local_addr: None,
        }
        .start()
    }
}

// --------------------------------------------------------------------------------//
// Message handlers

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetNetworkAddr(pub Addr<NetworkActor>);
impl Handler<SetNetworkAddr> for StoreStockActor {
    type Result = ();

    fn handle(&mut self, msg: SetNetworkAddr, ctx: &mut Context<Self>) -> Self::Result {
        println!("StoreStockActor: Binding StoreStockActor to NetworkActor");
        let net_addr = msg.0;

        // send message to NetworkActor to bind to self
        let res = net_addr.try_send(NetworkSetStoreStockAddr(ctx.address()));
        if res.is_err() {
            println!("StoreStockActor: NetworkActor is not alive");
        }

        self.network_addr = Some(net_addr);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetLocalAddr(pub Addr<LocalActor>);
impl Handler<SetLocalAddr> for StoreStockActor {
    type Result = ();

    fn handle(&mut self, msg: SetLocalAddr, ctx: &mut Context<Self>) -> Self::Result {
        println!("StoreStockActor: Binding StoreStockActor to LocalActor");
        let local_addr = msg.0;

        // send message to LocalActor to bind to self
        let res = local_addr.try_send(LocalSetStoreStockAddr(ctx.address()));
        if res.is_err() {
            println!("StoreStockActor: LocalActor is not alive");
        }
        self.local_addr = Some(local_addr);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Buy(pub &'static str);
impl Handler<Buy> for StoreStockActor {
    type Result = ();

    fn handle(&mut self, msg: Buy, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "StoreStockActor: chequeo stock de {}\nuhhh justo ese no lo tengo",
            msg.0
        );

        if let Some(network_addr) = &self.network_addr {
            _ = network_addr.try_send(NetworkBuy(msg.0));
            println!("Ahi consulte en el local de La Matanza a ver que dicen");
        } else {
            println!("Llame al local de La Matanza pero no me atendieron");
        }
    }
}
