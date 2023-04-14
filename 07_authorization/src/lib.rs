use scrypto::prelude::*;

#[blueprint]
mod authorization_module {

    struct Authorization {
        badge_container: Vault,
        fun_token_address: ResourceAddress,
    }

    impl Authorization {
        pub fn instantiate_authorization() -> ComponentAddress {
            // Create a fungible resource named Minting Badge with a a given supply of 1
            let minting_admin_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Minting Admin Badge")
                .metadata("description", "Minting Authority For Fun Tokens")
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            // create another fungible resource name Fun Token with a given supply of 1000 which is only mintable by a minting_admin_badge holder
            let fun_tokens = ResourceBuilder::new_fungible()
                .metadata("name", "Fun Tokens")
                .mintable(
                    rule!(require(minting_admin_badge.resource_address())),
                    rule!(deny_all),
                )
                .mint_initial_supply(1000);

            let component = Self {
                badge_container: Vault::with_bucket(minting_admin_badge),
                fun_token_address: fun_tokens.resource_address(),
            }
            .instantiate()
            .globalize();

            component
        }

        pub fn mint_fun_tokens(&self, amount: Decimal) -> Bucket {
            // creates a proof for badge resource and submits it to AuthZone.
            // If proof is validated then the logic defined in the closure is executed
            let fun_tokens = self.badge_container.authorize(|| {
                let resource_manager = borrow_resource_manager!(self.fun_token_address);
                resource_manager.mint(amount)
            });

            // a bucket of x amount of fun tokens will be trasferred to the transaction initiator and set into their vault
            return fun_tokens;
        }
    }
}
