use scrypto::prelude::*;

#[blueprint]
mod flat_admin_mocule {
    struct FlatAdmin {
        // Define what resources and data will be managed by Hello components
        admin_mint_badge: Vault,
        admin_badge: ResourceAddress,
    }

    impl FlatAdmin {
        pub fn instantiate_flat_admin(badgeName: String) -> ComponentAddress {
            // create badge for mint/burn
            // hold the authority to create admin_badges
            let admin_mint_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Admin_Mint_Badge")
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            // Create the ResourceManager for a mutable supply admin badge
            // access rules cannot be changed forever due to LOCKED

            let admin_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Admin_Badge")
                .divisibility(DIVISIBILITY_NONE)
                .mintable(rule!(require(admin_mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(admin_mint_badge.resource_address())), LOCKED)
                .create_with_no_initial_supply();

            let example_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Admin_Badge")
                .divisibility(DIVISIBILITY_NONE)
                .mintable(rule!(require(admin_mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(admin_mint_badge.resource_address())), LOCKED)
                .create_with_no_initial_supply();

            // using our minting badge authority, mint a single admin badge
            let mint_admin_badges = admin_mint_badge.authorize(|| {
                let admin_badge_manager = borrow_resource_manager!(admin_badge);

                admin_badge_manager.mint(1)
            });

            // Setting up the access rules of the component
            let access_rules_config = AccessRulesConfig::new()
                .method(
                    "create_additional_admin",
                    rule!(require(admin_badge)),
                    LOCKED,
                )
                .default(rule!(allow_all), LOCKED);

            let flat_admin_component = Self {
                admin_mint_badge: Vault::with_bucket(admin_mint_badge),
                admin_badge,
            }
            .instantiate()
            .globalize_with_access_rules(access_rules_config);

            // Return the instantiated component and the admin badge we just minted
            return (flat_admin_component, mint_admin_badges);
        }

        pub fn create_additional_admin(&mut self) -> Bucket {
            self.admin_mint_badge.authorize(|| {
                let admin_badge_manager = borrow_resource_manager!(self.admin_badge);

                admin_badge_manager.mint(1)
            })
        }

        pub fn destroy_admin_badge(&mut self, to_destroy: Bucket) {
            assert!(
                to_destroy.resource_address() == self.admin_badge,
                "Can not destroy the contents of this bucket!"
            );
            self.admin_mint_badge.authorize(|| {
                to_destroy.burn();
            });
        }
    }
}
