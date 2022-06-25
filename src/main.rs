use crate::auction_house::AuctionHouse;

mod auction_house;
mod utils;

#[tokio::main]
async fn main() {
    println!("Starting stonks ...");

    let auction_house = AuctionHouse::new(60);

    let (tx, mut rx) = tokio::sync::mpsc::channel(60);

    let handle = tokio::task::spawn(|| async move {
        while let Some(page_resp) = rx.recv().await {
            dbg!(page_resp)
        }
    });

    auction_house.collect_auctions(2, |pages| async move {
        for page_response in pages {
            tx.send(page_response).await;
        }
    }).await;

    handle.await;
}
