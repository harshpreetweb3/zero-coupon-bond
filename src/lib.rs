use scrypto::prelude::*;

#[blueprint]
mod zerocouponbond {

    struct ZeroCouponBond {
        contract_type: String,
        contract_role: String,
        contract_identifier: String,
        nominal_interest_rate: Decimal,
        // day_count_convention: String,
        currency: String,
        initial_exchange_date: u64,
        maturity_date: u64,
        notional_principal: Decimal,
        discount: u64,
    }

    impl ZeroCouponBond {
        // PAM issuer CONTRACT1234 0.04 30/360 USD 4724 4729 1000 100

        // No of bonds being issued by the bond issuer / creator ?
        // I think we will make bond a Resource (either Fungible / Non Fungible)
        // Bonds would be nothing but NFTs/ you can say tokens
        // like gumball, bond creator will set the price and mint some amount of bonds so that they can be purchased by the investors
        // XRD will be paid to purchase/invest in the bonds
        // Token will represent a bond
        // Token would be of some face value eg 1000; we do have to give some face value to the bond

        pub fn instantiate_zerocouponbond(
            contract_type: String,          // PAM
            contract_role: String,          // issuer
            contract_identifier: String,    // unique id for a contract
            nominal_interest_rate: Decimal, // how much interest is being given by a bond
            // day_count_convention: String,   // 30/360; this needs to be optimized for us  ; on hold
            currency: String,               // for example : USD
            initial_exchange_date: u64,     // initial exchange date
            maturity_date: u64,             // date when bond matures
            notional_principal: Decimal,    // price defined by bond creator
            discount: u64,                  // discount on bond
        ) -> Global<ZeroCouponBond> {
            Self {
                    contract_type,
                    contract_role,
                    contract_identifier,
                    nominal_interest_rate,
                    currency,
                    initial_exchange_date,
                    maturity_date,
                    notional_principal,
                    discount,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn get_contract_details(
            &self,
        ) -> (
            String,
            String,
            String,
            Decimal,
            // String,
            String,
            u64,
            u64,
            Decimal,
            u64,
        ) {
            (
                self.contract_type.clone(),
                self.contract_role.clone(),
                self.contract_identifier.clone(),
                self.nominal_interest_rate,
                // self.day_count_convention.clone(),
                self.currency.clone(),
                self.initial_exchange_date,
                self.maturity_date,
                self.notional_principal,
                self.discount,
            )
        }

        // function to check the price of the bond

        // there might be a possibility that Bond price will increase/ decrease in the market
        // what factors are resposible for changes in bond price and how will bond price be changed

        // function to purchase a bond

        // function to sell the bond (NFT) after the maturity period
        // To whom it will be sold ?

        // track after maturity period


    }
}


// idea
// instead of the IED and MD, we can take maturity_period such as 3M/90D, then after 3M/90D, amount will be repaid ... 
