//! spotlight

#[macro_use]
extern crate pbc_contract_codegen;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::{ContractContext};


/// This is the state of the contract which is persisted on chain.
///
/// The #\[state\] macro generates serialization logic for the struct.
///
/// ### Fields:
///
///   * `owner`: [`Address`], the owner of the contract.
///
///   * `value`: [`u64`], variable to illustrate reassignment of variable.
///
#[state]
struct ContractState {
    owner: Address,
    value: u64,
}

impl ContractState {
    fn set_value(&mut self, value: u64) {
        self.value = value;
    }
}


/// Initial function to bootstrap the contract's state.
///
/// # Parameters
///
///   * `ctx`: [`ContractContext`] - the contract context containing sender and chain information.
///
///   * `initial_value`: [`u64`] - the initial value.
///
/// # Returns
///
/// The new state object of type [`ContractState`] with an initial `state.value`.
///
#[init]
fn initialize(
    ctx: ContractContext,
    initial_value: u64,
) -> ContractState {

    let state = ContractState {
        owner: ctx.sender,
        value: initial_value,
    };

    state
}

/// This is the main action of the contract in which the sender can change the state value.
/// The only `Address` that can change the value is the owner of the contract.
///
/// # Parameters
///
///   * `ctx`: [`ContractContext`] - the contract context containing sender and chain information.
///
///   * `state`: [`ExampleContractState`], the current state of the contract.
///
///   * `value`: [`u64`] - the new value
///
/// # Returns
///
/// The return value is the new state with the updated value.
///
#[action(shortname = 0x01)]
fn set_value(
    context: ContractContext,
    state: ContractState,
    value: u64,
) -> ContractState {
    assert!(context.sender == state.owner, "Only the owner of the contract can change the value");
    assert!(value > 0, "Value must be > 0");

    let mut new_state = state;
    new_state.set_value(value);

    new_state
}
