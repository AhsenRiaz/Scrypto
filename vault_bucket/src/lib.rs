use scrypto::prelude::*;

#[blueprint]
mod vault_bucket {
    struct Token {
        sample_vault: Vault,
    }

    impl Token {
        pub fn instantiate_hello() -> ComponentAddress {
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "HelloToken")
                .metadata("symbol", "HT")
                .mint_initial_supply(1000);

            Self {
                sample_vault: Vault::with_bucket(my_bucket),
            }
            .instantiate()
            .globalize()
        }

        pub fn accept_tokens(&mut self, bucket: Bucket) {
            assert_eq!(
                bucket.resource_address(),
                self.sample_vault.resource_address()
            );
            self.sample_vault.put(bucket);
        }

        pub fn receive_tokens(&mut self, amount: Decimal) -> Bucket {
            let return_tokens = self.sample_vault.take(amount);
            return_tokens
        }

        pub fn buy_something(&mut self, mut bucket: Bucket) -> Bucket {
            let price = dec!(10);

            assert!(bucket.amount() >= price, "Not enough tokens sent");

            let payment = bucket.take(price);

            self.sample_vault.put(payment);

            bucket
        }

        pub fn burn(&mut self, bucket: Bucket) {
            assert!(bucket.resource_address() == self.sample_vault.resource_address());

            bucket.burn()
        }
    }
}
