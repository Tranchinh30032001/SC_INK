#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod hkt_plats {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    #[ink(storage)]
    pub struct HktPlats {
        owner: AccountId,
        participants: Mapping<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Deposited {
        #[ink(topic)]
        from: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Rewarded {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        amount: Balance,
    }

    impl HktPlats {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                participants: Mapping::new(),
            }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self, amount: Balance) {
            let balance = self.env().transferred_value();
            let sender = self.env().caller();
            assert!(balance == amount, "The amount must be equal to the balance");
            let old_balance = self.participants.get(sender).unwrap_or(0);
            self.participants.insert(sender, &(old_balance + amount));
            self.env().emit_event(Deposited {
                from: sender,
                amount,
            })
        }

        #[ink(message)]
        pub fn reward(&mut self, lucky_user: Vec<AccountId>) {
            let length_person = u128::try_from(lucky_user.len()).unwrap();
            let caller = self.env().caller();
            let balance_caller = self.participants.get(caller).unwrap_or(0);
            assert!(
                balance_caller > 0,
                "The balance of caller must be greater than zero"
            );
            let token_per_person = balance_caller / length_person;
            for user in lucky_user.iter() {
                let res = self.env().transfer(user.clone(), token_per_person);
                match res.ok() {
                    Some(_) => self.env().emit_event(Rewarded {
                        from: caller,
                        to: *user,
                        amount: token_per_person,
                    }),
                    None => panic!("rewarded failed"),
                }
            }
        }
    }
}
