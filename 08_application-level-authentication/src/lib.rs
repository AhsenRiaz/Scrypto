use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
struct MyNFTData {
    #[mutable]
    xrd_claimed: bool,
}

#[blueprint]
mod simple_nft_project {
    struct SimpleNFTProject {
        nft_resource: ResourceAddress,
        vault: Vault,
        admin_badge: Vault,
    }

    impl SimpleNFTProject {
        pub fn instantiate_simple_nft_project(funds: Bucket) -> ComponentAddress {
            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            let nft_resource = ResourceBuilder::new_uuid_non_fungible::<MyNFTData>()
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(admin_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply();

            Self {
                nft_resource,
                admin_badge: Vault::with_bucket(admin_badge),
                vault: Vault::with_bucket(funds),
            }
            .instantiate()
            .globalize()
        }

        pub fn mint_nft(&mut self) -> Bucket {
            let nft = self.admin_badge.authorize(|| {
                let nft_resource_manager = borrow_resource_manager!(self.nft_resource);

                nft_resource_manager.mint_uuid_non_fungible(MyNFTData { xrd_claimed: false })
            });

            return nft;
        }

        pub fn claim_xrd(&mut self, auth: Proof) -> Bucket {
            // application-level-authentication
            let auth = auth
                .validate_proof(self.nft_resource)
                .expect("Invalid NFT Passed");

            let nft: NonFungible<MyNFTData> = auth.non_fungible();

            let mut nft_data = nft.data();

            assert!(
                !nft_data.xrd_claimed,
                "You have already claimed you 10 XRD coins"
            );

            self.admin_badge.authorize(|| {
                let nft_resource_manager = borrow_resource_manager!(self.nft_resource);

                nft.update_data(MyNFTData { xrd_claimed: true });
            });

            return self.vault.take(10)
        }
    }
}
