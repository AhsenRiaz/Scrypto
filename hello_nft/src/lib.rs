use scrypto::prelude::*;

#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe)]
pub enum Color {
    White,
    Blue,
    Red,
    Black,
    Green,
}

#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    MythicRare,
}

#[derive(NonFungibleData)]
pub struct MagicCard {
    color: Color,
    rarity: Rarity,
    #[mutable]
    level: u8,
}

#[blueprint]
mod hello_nft {
    struct HelloNFT {
        // a vault that holds all our special cards
        special_cards: Vault,
        // price of each special card
        special_card_price: HashMap<NonFungibleLocalId, Decimal>,
        // a vault that holds the mint badge
        random_card_mint_badge: Vault,
        // resource address for the random card resource
        random_card_resource_address: ResourceAddress,
        // price of each random card
        random_card_price: Decimal,
        // random card id counter
        random_card_id_counter: u64,
        // a vault that collects all the xrd payments
        collected_xrd: Vault,
    }

    impl HelloNFT {
        pub fn instantiate_component() -> ComponentAddress {
            // Creates a fixed set of NFTs
            let special_cards_bucket = ResourceBuilder::new_integer_non_fungible()
                .metadata("Batman ", "Batman Card Collection")
                .mint_initial_supply([
                    (
                        IntegerNonFungibleLocalId::new(1u64),
                        MagicCard {
                            color: Color::Black,
                            rarity: Rarity::MythicRare,
                            level: 3,
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(2u64),
                        MagicCard {
                            color: Color::Green,
                            rarity: Rarity::Rare,
                            level: 5,
                        },
                    ),
                    (
                        IntegerNonFungibleLocalId::new(3u64),
                        MagicCard {
                            color: Color::Red,
                            rarity: Rarity::Uncommon,
                            level: 100,
                        },
                    ),
                ]);

            // Mint a badge - used for authorization
            let random_card_mint_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Random Cards Mint Badge")
                .mint_initial_supply(1);

            let random_card_resource_address = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Random Cards")
                .mintable(
                    rule!(require(random_card_mint_badge.resource_address())),
                    LOCKED,
                )
                .burnable(
                    rule!(require(random_card_mint_badge.resource_address())),
                    LOCKED,
                )
                .updateable_non_fungible_data(
                    rule!(require(random_card_mint_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply();

            // Instantiate Our Component
            Self {
                special_cards: Vault::with_bucket(special_cards_bucket),
                special_card_price: HashMap::from([
                    (NonFungibleLocalId::Integer(1u64.into()), 100.into()),
                    (NonFungibleLocalId::Integer(2u64.into()), 100.into()),
                    (NonFungibleLocalId::Integer(3u64.into()), 100.into()),
                ]),
                random_card_mint_badge: Vault::with_bucket(random_card_mint_badge),
                random_card_resource_address,
                random_card_price: 50.into(),
                random_card_id_counter: 0,
                collected_xrd: Vault::new(RADIX_TOKEN),
            }
            .instantiate()
            .globalize()
        }

        // buy special card nft for its price
        pub fn buy_special_card(
            &mut self,
            key: NonFungibleLocalId,
            mut payment: Bucket,
        ) -> (Bucket, Bucket) {
            // unwrap handles the error when a key does not exist in the hashmap
            // removes the key value and returns the value associated to the key
            let price = self.special_card_price.remove(&key).unwrap();

            // takes the payment from the bucket and adds the payment into collected_xrd vault
            self.collected_xrd.put(payment.take(price));

            // Take the requested NFT
            let nft_bucket = self.special_cards.take_non_fungible(&key);

            // return the nft and the change
            (nft_bucket, payment)
        }

        pub fn buy_random_card(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            self.collected_xrd.put(payment.take(self.random_card_price));

            // Mint a new card
            let random_seed = 100;

            let new_card = MagicCard {
                color: Self::random_color(random_seed),   // returns color
                rarity: Self::random_rarity(random_seed), // returns rarity
                level: random_seed as u8 % 8,             // level
            };

            let nft_bucket = self.random_card_mint_badge.authorize(|| {
                borrow_resource_manager!(self.random_card_resource_address).mint_non_fungible(
                    &NonFungibleLocalId::integer(self.random_card_id_counter),
                    new_card,
                )
            });

            self.random_card_id_counter += 1;

            // Return the NFT and change
            (nft_bucket, payment)
        }

        pub fn upgrade_my_random_card(&mut self, nft_bucket: Bucket) -> Bucket {
            assert!(
                nft_bucket.amount() == dec!("1"),
                "We can only upgrade one card each time"
            );

            // get and update the mutable data
            let mut non_fungible_data: MagicCard = nft_bucket.non_fungible().data();

            non_fungible_data.level += 1;

            self.random_card_mint_badge
                .authorize(|| nft_bucket.non_fungible().update_data(non_fungible_data));

            nft_bucket
        }

        fn fuse_my_cards(&mut self, nft_bucket: Bucket) -> Bucket {
            assert!(
                nft_bucket.amount() == dec!("2"),
                "You need to pass 2 NFTs for fusion"
            );

            assert!(
                nft_bucket.resource_address() == self.random_card_resource_address,
                "Only random cards can be fused",
            );

            // Retrieve the NFT data
            let card1 = nft_bucket.non_fungibles()[0].data();
            let card2 = nft_bucket.non_fungibles()[1].data();

            let new_card = Self::fuse_magic_cards(card1, card2);

            // burn the original cards

            self.random_card_mint_badge.authorize(|| nft_bucket.burn());

            // Mint a new one
            let new_non_fungible_bucket = self.random_card_mint_badge.authorize(|| {
                borrow_resource_manager!(self.random_card_resource_address).mint_non_fungible(
                    &NonFungibleLocalId::Integer(self.random_card_id_counter.into()),
                    new_card,
                )
            });

            self.random_card_id_counter += 1;

            new_non_fungible_bucket
        }

        fn random_color(seed: u64) -> Color {
            match seed % 5 {
                0 => Color::White,
                1 => Color::Blue,
                2 => Color::Black,
                3 => Color::Red,
                4 => Color::Green,
                _ => panic!(),
            }
        }

        fn random_rarity(rarity: u64) -> Rarity {
            match rarity % 4 {
                0 => Rarity::Common,
                1 => Rarity::Uncommon,
                2 => Rarity::Rare,
                3 => Rarity::MythicRare,
                _ => panic!(),
            }
        }

        fn fuse_magic_cards(card1: MagicCard, card2: MagicCard) -> MagicCard {
            MagicCard {
                color: card1.color,
                rarity: card2.rarity,
                level: card1.level + card2.level,
            }
        }
    }
}
