use crate::auction_house::AuctionHouse;

mod auction_house;
mod utils;

#[tokio::main]
async fn main() {
    println!("Starting stonks ...");

    let auction_house = AuctionHouse::new(6);

    auction_house.collect_auctions(10, |pages| async move {
        for page in pages {
            dbg!(page.page);
        }
    }).await;
}
