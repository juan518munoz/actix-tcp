use actix::prelude::*;

use crate::store_stock::actor::{Buy as StoreStockBuy, StoreStockActor};

pub struct LocalActor {
    store_stock_addr: Option<Addr<StoreStockActor>>,
}

impl Actor for LocalActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("LocalActor is alive");
    }
}

impl LocalActor {
    pub fn new() -> Addr<LocalActor> {
        LocalActor {
            store_stock_addr: None,
        }
        .start()
    }
}

// --------------------------------------------------------------------------------//
// Message handlers

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetStoreStockAddr(pub Addr<StoreStockActor>);
impl Handler<SetStoreStockAddr> for LocalActor {
    type Result = ();

    fn handle(&mut self, msg: SetStoreStockAddr, _ctx: &mut Context<Self>) -> Self::Result {
        println!("LocalActor: Binding StoreStockActor to LocalActor");
        self.store_stock_addr = Some(msg.0);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Buy(pub &'static str);
impl Handler<Buy> for LocalActor {
    type Result = ();

    fn handle(&mut self, msg: Buy, _ctx: &mut Context<Self>) -> Self::Result {
        println!("LocalActor: Alguien quiere comprar {}", msg.0);

        // send message to StoreStockActor requesting to buy
        if let Some(store_stock_addr) = &self.store_stock_addr {
            println!("Pera que consulto stock");
            _ = store_stock_addr.try_send(StoreStockBuy(msg.0));
        } else {
            println!("No hay sistema pa");
        };
    }
}
