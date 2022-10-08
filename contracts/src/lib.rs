mod models;
mod utils;
use crate::{
    models::{Crowdfund, Donation},
    utils::{assert_self, assert_single_promise_success, ONE_NEAR, AccountId},
};

use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}, Promise};
#[allow(unused_imports)]
use near_sdk::{env, PromiseIndex, near_bindgen};

#[near_bindgen]
// bindgen allows the contract to be read
// by near after webAss compilation

#[derive(Clone,Default,BorshDeserialize,BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    donations: Vec<Donation>,
    crowdfunds: Vec<Crowdfund>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner: AccountId) -> Self {
        let crowdfunds: Vec<Crowdfund> = Vec::new();
        let donations: Vec<Donation> = Vec::new();

        Contract { owner, donations, crowdfunds }
    }
    
    pub fn add_crowdfund(&mut self, title: String, donation_target:u128, description:String) {
        let id = self.crowdfunds.len() as i32;
    
        self.crowdfunds.push(Crowdfund::new(id, title, donation_target, description));
        env::log_str("Added a new crowdfund");
    }

    pub fn list_crowdfunds(&self) -> Vec<Crowdfund> {
        assert_self();
        // why this exists no idea, possibly assert owner instead.

        let crowdfunds = &self.crowdfunds;
        return crowdfunds.to_vec();
    }

    pub fn add_vote(&mut self, id:usize) {
        let crowdfund = self.crowdfunds.get_mut(id).unwrap();
        let voter = env::predecessor_account_id();
        crowdfund.total_votes = crowdfund.total_votes + 1;
        env::log_str("vote success");
        crowdfund.votes.push(voter.to_string());
    }

    pub fn add_donation(&mut self, id:usize, amount:u128) {
        let transfer_amount = ONE_NEAR * amount;
        let crowdfund = self.crowdfunds.get_mut(id).unwrap();
        crowdfund.total_donations = crowdfund.total_donations + transfer_amount;
        self.donations.push(Donation::new());

        Promise::new(env::predecessor_account_id()).transfer(transfer_amount);
        // dumb because this goes back to the transaction signer
        // Also drains funds from the contract itself and not the caller.
        // Dumb, need to update this.

        env::log_str("money sent");
        // Where does the money go....to the contract?
    }

    pub fn crowdfund_count(&mut self) -> usize {
        return self.crowdfunds.len()
    }

    pub fn get_total_donations(&mut self, id:usize) -> u128 {
        let crowdfund = self.crowdfunds.get_mut(id).unwrap();
        return crowdfund.total_donations;
    }
}

// https://docs.near.org/tutorials/examples/donation
// near call crowdfund1.ctorra.testnet add_crowdfund '{"title": "Eliots eye sight", "donation_target": 30, "description":"Raise funds for little Eliot to see again. Loss of sight was caused by an accident to the head"}' --accountId ctorra.testnet
// near call crowdfund1.ctorra.testnet add_vote '{"id":0}' --accountId ctorra.testnet
// near call crowdfund1.ctorra.testnet add_donation '{"id":0, "amount":1}' --accountId ctorra.testnet
// near call crowdfund1.ctorra.testnet list_crowdfunds --accountId crowdfund1.ctorra.testnet
