mod utils;

use near_contract_standards::upgrade::Ownable;
// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};
use utils::access_control::AccessControl;

setup_alloc!();

#[near_bindgen]
#[derive( BorshDeserialize, BorshSerialize )]
pub struct Product {
    records: LookupMap<String, Item>,
    owner: AccountId,
    access: AccessControl,
}

const ROLE_SET_PRODUCT:&str = "ROLE_SET_PRODUCT";
const ROLE_DELETE_PRODUCT:&str = "ROLE_DELETE_PRODUCT";

#[derive(Debug)]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Item {
     name: String,
     price: u128,
     stock: u8
}

impl Default for Product {
    fn default() -> Self {
        env::panic(b"Product contract should be initialized before usage")
    }
}
// set owner
impl Ownable for Product{
    fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner = owner;
    }
}
 
// implements access control
#[near_bindgen]
impl Product{
    pub fn add_role_set_product(&mut self, account: AccountId ){
        //validate if owner
        self.assert_owner();
        self.access.setup_role(ROLE_SET_PRODUCT.to_string(), account);
    }

    pub fn add_role_delete_product(&mut self, account: AccountId ){
        //validate if owner
        self.assert_owner();
        self.access.setup_role(ROLE_DELETE_PRODUCT.to_string(), account);
    }

}

// management products
#[near_bindgen]
impl Product{

    #[init]
    pub fn new()-> Self{
        assert!(!env::state_exists(), "The contract is already initialized");
      

       let mut this = Self{
            records: LookupMap::new(b"a".to_vec()),
            owner: env::signer_account_id(),
            access: AccessControl { roles: LookupMap::new(b"a".to_vec()) },
        };

        this.add_role_set_product(env::signer_account_id());

        this.add_role_delete_product(env::signer_account_id());
        
        this
    }


    pub fn set_products(&mut self, address:String, name:String, price: u128, stock:u8){
        
        //validate sender has permition of ROLE_SET_PRODUCT
        assert_eq!(self.access.has_role(&ROLE_SET_PRODUCT.to_string(), &env::signer_account_id()), true, "401");
        let item = Item {name, price, stock};
        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("set_product '{:?}' ", item).as_bytes());
        self.records.insert(&address, &item);
    }

    pub fn get_products(&self, address:String) -> Option<Item>{
         self.records.get(&address)
    }

    pub fn delete_products(&mut self, address:String) {
       
         //validate sender has permition of ROLE_DELETE_PRODUCT
        assert_eq!(self.access.has_role(&ROLE_DELETE_PRODUCT.to_string(), &env::signer_account_id()), true, "401");
        
        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("delete_products '{}' ", address).as_bytes());
        self.records.remove(&address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "Paul".to_string(),
            signer_account_id: "Paul".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "Paul".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_product() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Product::new();
         
        contract.set_products("0x1".to_string(), "PS4 x".to_string(), 800, 100);
       
       let result = contract.get_products("0x1".to_string());
       
       let val = match result {
            // The division was valid
            Some(x) => {
                println!("Result: name:{}, price:{}, stock:{}", x.name, x.price, x.stock);
                x.price
            },
            // The division was invalid
            None    => {
                println!("Not Exists");
                0
            },
          
        };

        assert_eq!(800, val );
       
    }

    #[test]
    fn get_default_product() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let  contract = Product::new();
        
        let result = contract.get_products("0x1".to_string());
       
        let val = match result {
             Some(x) => {
                println!("Result: name:{}, price:{}, stock:{}", x.name, x.price, x.stock);
                 x.price
             },
             None    => {
                 println!("Not Exists");
                 0
             },
           
         };

        assert_eq!(
           0,
           val
        );
    }

    #[test]
    fn set_delete_product() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Product::new();
       
        contract.set_products("0x11".to_string(), "PS5".to_string(),12345, 12);
       
       
        contract.delete_products("0x11".to_string());
       
       let result = contract.get_products("0x11".to_string());
       
       let val = match result {
            // The division was valid
            Some(x) => {
                println!("Result: name:{}, price:{}, stock:{}", x.name, x.price, x.stock);
                x.price
            },
            // The division was invalid
            None    => {
                println!("Not Exists");
                0
            },
          
        };

        assert_eq!(0, val );
       
    }

    #[test]
    fn update_get_product() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Product::new();
       
        contract.set_products("0x1".to_string(), "PS5".to_string(),500, 12);
       
        contract.set_products("0x1".to_string(), "PS5".to_string(),1200, 7);
       
       let result = contract.get_products("0x1".to_string());
       
       let val = match result {
            // The division was valid
            Some(x) => {
                println!("Result: name:{}, price:{}, stock:{}", x.name, x.price, x.stock);
                x.stock
            },
            // The division was invalid
            None    => {
                println!("Not Exists");
                0
            },
          
        };

        assert_eq!(7, val );
       
    }
}
