use std::future::Future;
use futures::{stream, StreamExt};
use reqwest::{Client, Response};

use serde::{Serialize, Deserialize};
use tokio_retry::Retry;
use tokio_retry::strategy::ExponentialBackoff;

const MAX_CONCURRENT_REQUESTS: usize = 750;
const MAX_FAILS: usize = 5;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionPage {
    pub success: bool,
    pub page: i64,
    pub total_pages: i64,
    pub total_auctions: i64,
    pub last_updated: i64,
    pub auctions: Vec<Auction>,
}

#[derive(Debug, Deserialize)]
pub struct Auction {
    pub uuid: String,
    pub item_name: String,
    pub tier: String,
    pub starting_bid: i64,
    pub item_bytes: String,
    pub claimed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<bool>,
}

pub struct AuctionHouse {
    pub base_url: String,
    pub total_pages: i64
}

impl AuctionHouse {
    pub fn new(total_pages: i64) -> Self {
        Self {
            base_url: "https://api.hypixel.net/skyblock/auctions?page=".to_string(), // concat right page index at the end
            total_pages
        }
    }

    pub async fn collect_auctions<Fut: Future<Output = ()>>(
        &self,
        f: impl FnMut(AuctionPage) -> Fut,
    ) {
        let client = reqwest::Client::new();
        let token_ids = (0..self.total_pages).into_iter();
        stream::iter(token_ids)
            .map(|page_nb| {
                let client = client.clone();
                let url = self.base_url.clone();
                tokio::spawn(async move {
                    Self::get_page_from_url(&client, page_nb, &url).await.unwrap()
                })
                // TODO: (maybe) HANDLE ERROR
            })
            .buffer_unordered(MAX_CONCURRENT_REQUESTS)
            .filter_map(|res| async { res.ok() })
            .for_each(f)
            .await;
    }

    pub async fn get_page(
        &self,
        client: &reqwest::Client,
        page_nb: i64,
    ) -> Result<AuctionPage, reqwest::Error> {
        Self::get_page_from_url(client, page_nb, &self.base_url).await
    }

    pub async fn get_page_from_url(
        client: &reqwest::Client,
        page_nb: i64,
        api_url: &str,
    ) -> Result<AuctionPage, reqwest::Error> {
        let url = format!("{}{}", api_url, page_nb);
        let retry_strategy = ExponentialBackoff::from_millis(100).take(MAX_FAILS);
        Retry::spawn(retry_strategy, || async {
            let resp = client.get(&url).send().await?;
            dbg!(resp.status());
            resp.json::<AuctionPage>().await
        })
            .await
    }
}
