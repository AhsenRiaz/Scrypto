use scrypto::prelude::*;

#[blueprint]
mod resource_builder_behaviors {

    struct ResourceBehaviors {
        mintable_vault: Vault,
        burnable_vault: Vault,
        non_withdrawable_vault: Vault,
        soul_bound_vault: Vault,
        updateable_nft_vault: Vault,
        rentable_vault: Vault,
    }

    impl ResourceBehaviors {
        pub fn instantiate_component() {
            // mintable resource behavior
            ResourceBuilder::new_fungible()
                .metadata("name", "DemoToken")
                .metadata("symbol", "DEMO")
                // We are adding the mintable flag here
                .mintable(rule!(allow_all), LOCKED)
                .create_with_no_initial_supply();

            // burnable resource behavior
            ResourceBuilder::new_fungible()
                .metadata("name", "DemoToken")
                .metadata("symbol", "DEMO")
                .mintable(rule!(allow_all), LOCKED)
                // We are adding the burnable flag here
                .burnable(rule!(allow_all), LOCKED)
                .create_with_no_initial_supply();

            // non_withdrawable resource behavior (usecase as a soulbound token)
            ResourceBuilder::new_fungible()
                .metadata("name", "DemoToken")
                .metadata("symbol", "DEMO")
                // We are adding the restrict_withdraw flag here
                .restrict_withdraw(rule!(allow_all), LOCKED)
                .create_with_no_initial_supply();

            // non_depositable resourc behavior (usecase- burn tokens on a certain event)
            ResourceBuilder::new_fungible()
                .metadata("name", "DemoToken")
                .metadata("symbol", "DEMO")
                // We are adding the restrict_deposit flag here
                .restrict_deposit(rule!(allow_all), LOCKED)
                .create_with_no_initial_supply();

            // updateable_nft resource behavior
            ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "DemoToken")
                .metadata("symbol", "DEMO")
                // We are adding the update NF data flag here
                .updateable_non_fungible_data(rule!(allow_all), LOCKED)
                .create_with_no_initial_supply();

            // recallable resource behavior (usecase as rental nfts)
            ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "DemoToken")
                .metadata("symbol", "DEMO")
                // We are adding the recallable flag here
                .recallable(rule!(allow_all), LOCKED)
                .create_with_no_initial_supply();
        }
    }
}
