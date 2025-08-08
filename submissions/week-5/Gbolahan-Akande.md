### Task: stylus-erc20
**GitHub:** https://github.com/gboigwe/stylus-erc20
**Summary:**  
Implemented a complete ERC-20 token standard in Rust for Arbitrium Stylus. The contract follows the OpenZeppelin ERC-20 specification with all required functions and events. Updated implementation to work with the latest Stylus SDK v0.9.0, addressing API changes including deprecated msg::sender() and evm::log() functions.
**Features:**  
- Core ERC-20 functions: `name`, `symbol`, `decimals`, `totalSupply`, `balanceOf`, `transfer`, `approve`, `allowance`, `transferFrom`
- Events: `Transfer(from, to, value)`, `Approval(owner, spender, value)`
- Optional `mint` function for token creation
- Comprehensive unit tests covering transfers, allowances, and insufficient balance scenarios
- Compatible with Stylus SDK 0.9.0 using `sol_storage!` macro and `self.vm()` methods
**Commands:**
```bash
cargo stylus new erc20_token
cargo stylus build
cargo stylus check
cargo test