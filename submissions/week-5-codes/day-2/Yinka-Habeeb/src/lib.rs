#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use stylus_sdk::{evm, msg, prelude::*};

sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
}

#[derive(SolidityError)]
pub enum Erc20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}

sol_storage! {
    #[entrypoint]
    pub struct SimpleToken {
        string name;
        string symbol;
        uint256 decimals;
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
        address owner;
    }
}
impl SimpleToken {
    // Helper function to emit events only when not testing
    fn emit_transfer(&self, from: Address, to: Address, value: U256) {
        #[cfg(not(test))]
        evm::log(Transfer { from, to, value });
    }

    fn emit_approval(&self, owner: Address, spender: Address, value: U256) {
        #[cfg(not(test))]
        evm::log(Approval { owner, spender, value });
    }
}

#[public]
impl SimpleToken {
    pub fn init(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
    ) -> Result<(), Vec<u8>> {
        self.name.set_str(&name);
        self.symbol.set_str(&symbol);
        self.decimals.set(U256::from(decimals)); 
        
        let deployer = self.vm().msg_sender();
        self.owner.set(deployer);
        
        if initial_supply > U256::ZERO {
            self.balances.setter(deployer).set(initial_supply);
            self.total_supply.set(initial_supply);
            self.emit_transfer(Address::ZERO, deployer, initial_supply);
        }
        Ok(())
    }

    pub fn name(&self) -> String {
        self.name.get_string()
    }

    pub fn symbol(&self) -> String {
        self.symbol.get_string()
    }

    pub fn decimals(&self) -> U256 {
        self.decimals.get()
    }

    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    pub fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(owner)
    }

    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        self._transfer(self.vm().msg_sender(), to, value)?;
        Ok(true)
    }

    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Erc20Error> {
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(msg::sender());
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err(Erc20Error::InsufficientAllowance(InsufficientAllowance {
                owner: from,
                spender: self.vm().msg_sender(),
                have: old_allowance,
                want: value,
            }));
        }

        allowance.set(old_allowance - value);
        self._transfer(from, to, value)?;
        Ok(true)
    }

    pub fn approve(&mut self, spender: Address, value: U256) -> bool {
        self.allowances.setter(self.vm().msg_sender()).insert(spender, value);
        self.emit_approval(self.vm().msg_sender(), spender, value);
        true
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.getter(owner).get(spender)
    }

    pub fn mint(&mut self, to: Address, amount: U256) -> Result<bool, Vec<u8>> {
        if self.vm().msg_sender() != self.owner.get() {
            return Err(b"Only owner can mint".to_vec());
        }
        if to == Address::ZERO {
            return Err(b"Cannot mint to address zero".to_vec());
        }

        let current_balance = self.balances.get(to);
        self.balances.setter(to).set(current_balance + amount);
        self.total_supply.set(self.total_supply.get() + amount);

        self.emit_transfer(Address::ZERO, to, amount);
        Ok(true)
    }

    fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), Erc20Error> {
        let old_sender_balance = self.balances.get(from);
        if old_sender_balance < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from,
                have: old_sender_balance,
                want: value,
            }));
        }

        self.balances.setter(from).set(old_sender_balance - value);
        let old_recipient_balance = self.balances.get(to);
        self.balances.setter(to).set(old_recipient_balance + value);

        self.emit_transfer(from, to, value);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloy_primitives::address;
    use stylus_sdk::testing::*;

    #[test]
    fn test_token_initialization() {
        let vm = TestVMBuilder::new()
            .sender(address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"))
            .build();
        let mut contract = SimpleToken::from(&vm);
        
        let result = contract.init(
            "Test Token".to_string(),
            "TEST".to_string(),
            18,
            U256::from(1000000),
        );

        assert!(result.is_ok());
        assert_eq!(contract.name(), "Test Token");
        assert_eq!(contract.symbol(), "TEST");
        assert_eq!(contract.decimals(), U256::from(18));
        assert_eq!(contract.total_supply(), U256::from(1000000));
    }

    #[test]
    fn test_transfer() {
        let vm = TestVMBuilder::new()
            .sender(address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"))
            .build();
        let mut contract = SimpleToken::from(&vm);
        let user = address!("0xCDC41bff86a62716f050622325CC17a317f99404");
        
        // Initialize contract first
        contract.init("Test Token".to_string(), "TEST".to_string(), 18, U256::from(1000)).unwrap();
        
        // Test transfer
        let result = contract.transfer(user, U256::from(100));
        assert!(result.is_ok());
        
        // Check balance
        assert_eq!(contract.balance_of(user), U256::from(100));
    }
}