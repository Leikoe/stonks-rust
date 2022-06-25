use crate::auction_house::AuctionHouse;

mod auction_house;
mod utils;

#[tokio::main]
async fn main() {
    println!("Starting stonks ...");

    let auction_house = AuctionHouse::new(60);

    auction_house.collect_auctions(|page| async move {
        dbg!(page.page);
    }).await;
}
