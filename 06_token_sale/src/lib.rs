use scrypto::prelude::*;

#[blueprint]
mod token_sale_module {

    struct TokenSale {
        useful_tokens_vault: Vault,
        collected_xrd: Vault,
        price: Decimal,
    }
    impl TokenSale {
        // parameters: price -> price of token
        // function: creates a FungibleResource with fixed supply of 100
        pub fn instantiate_token_sale(price: Decimal) -> ComponentAddress {
            let bucket_of_useful_tokens = ResourceBuilder::new_fungible()
                .metadata("name", "UsefulTokens")
                .metadata("symbol", "UT")
                .metadata("description", "This is a really useful token")
                .mint_initial_supply(100);

            Self {
                useful_tokens_vault: Vault::with_bucket(bucket_of_useful_tokens),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price,
            }
            .instantiate()
            .globalize()
        }

        // buy 1 token
        pub fn buy_useful_token(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            // take payment from the bucket and add the payment into collected_xrd vaultI
            self.collected_xrd.put(payment.take(self.price));

            // send 1 token and return the remaning payment
            (self.useful_tokens_vault.take(1), payment)
        }

        // get price of token
        pub fn get_price(&self) -> Decimal {
            self.price
        }
    }
}
