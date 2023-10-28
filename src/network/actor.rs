use actix::prelude::*;

use crate::store_stock::actor::StoreStockActor;

pub struct NetworkActor {
    store_stock_addr: Option<Addr<StoreStockActor>>,
}

impl Actor for NetworkActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("NetworkActor is alive");
    }
}

impl NetworkActor {
    pub fn new() -> Addr<NetworkActor> {
        let net_addr = NetworkActor {
            store_stock_addr: None,
        }
        .start();

        let addr_clone = net_addr.clone();

        // MOVE THIS CODE TO tcp_handler.rs
        let _listener_thread = tokio::spawn(async move {
            loop {
                // send message to NetworkActor
                addr_clone.clone().send(TcpMsg).await.unwrap();
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });

        net_addr
    }
}

// --------------------------------------------------------------------------------//
// Message handlers

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetStoreStockAddr(pub Addr<StoreStockActor>);
impl Handler<SetStoreStockAddr> for NetworkActor {
    type Result = ();

    fn handle(&mut self, msg: SetStoreStockAddr, _ctx: &mut Context<Self>) -> Self::Result {
        println!("NetworkActor: Binding StoreStockActor to NetworkActor");
        self.store_stock_addr = Some(msg.0);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct TcpMsg;
impl Handler<TcpMsg> for NetworkActor {
    type Result = ();

    fn handle(&mut self, _msg: TcpMsg, _ctx: &mut Context<Self>) -> Self::Result {
        println!("NetworkActor: Received a message from the tcp stream");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Buy(pub &'static str);
impl Handler<Buy> for NetworkActor {
    type Result = ();

    fn handle(&mut self, msg: Buy, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "NetworkActor (La Matanza): comprar {}\nsisi, aca lo tenemos",
            msg.0
        );
    }
}
