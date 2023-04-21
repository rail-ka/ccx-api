use crate::api::exchange::prelude::*;
use crate::api::exchange::ProductTicker;
use crate::api::exchange::RL_PUBLIC_KEY;

pub type GetProductTickerResponse = ProductTicker;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get product ticker.
    ///
    /// Gets snapshot information about the last trade (tick), best bid/ask and 24h volume.
    ///
    ///
    /// Real-time updates
    ///
    /// Coinbase recommends that you get real-time updates by connecting with the WebSocket
    /// stream and listening for match messages, rather than polling.
    ///
    /// * `product_id` - .
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getproductticker]
    pub fn get_product_ticker(
        &self,
        product_id: Atom,
    ) -> CoinbaseResult<Task<GetProductTickerResponse>> {
        // let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("products/{product_id}/ticker");
        Ok(self
            .rate_limiter
            .task(self.client
                    .get(&endpoint)?
                    // .signed(timestamp)?
                    .request_body(())?)
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}