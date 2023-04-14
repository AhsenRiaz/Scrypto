use scrypto::prelude::*;

#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Debug)]
enum Vegetable {
    Tomato,
    Carrot,
    Brocolli,
}

#[blueprint]
mod exercise_1 {
    struct Exercise1 {
        instantiated_at: u64,
        instantiator_name: String,
        favourite_vegetable: Vegetable,
    }

    impl Exercise1 {
        pub fn instantiate_exercise(instantiator_name: String) -> ComponentAddress {
            Self {
                favourite_vegetable: Vegetable::Brocolli,
                instantiated_at: Runtime::current_epoch(),
                instantiator_name: instantiator_name,
            }
            .instantiate()
            .globalize()
        }

        pub fn log_data(&self) {
            error!(
                "Instantiated by {} at epoch {}",
                self.instantiator_name, self.instantiated_at
            );

            debug!("The favourite vegetable is {:?}", self.favourite_vegetable);
        }
    }
}
