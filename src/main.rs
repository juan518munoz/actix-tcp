pub mod local;
pub mod network;
pub mod store_stock;

use actix::prelude::*;
use local::actor::{Buy, LocalActor};
use network::actor::NetworkActor;
use store_stock::actor::{SetLocalAddr, SetNetworkAddr, StoreStockActor};

use tokio::time::{sleep, Duration};

#[actix_rt::main]
async fn main() {
    let store_stock_addr: Addr<StoreStockActor> = StoreStockActor::new();

    let network_addr: Addr<NetworkActor> = NetworkActor::new();
    _ = store_stock_addr // bind network actor to store_stock actor together
        .send(SetNetworkAddr(network_addr.clone()))
        .await;

    let local_addr: Addr<LocalActor> = LocalActor::new();
    _ = store_stock_addr // bind local actor to store_stock actor together
        .send(SetLocalAddr(local_addr.clone()))
        .await;

    println!("\n\n");
    sleep(Duration::from_secs(8)).await;

    // emulate customer making local request
    local_addr.send(Buy("'el cosito ese'")).await.unwrap();
}
