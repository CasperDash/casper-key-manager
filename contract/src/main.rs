#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// Explicitly import the std alloc crate and `alloc::string::String` as we're in a `no_std` environment.
extern crate alloc;

use alloc::string::String;
use alloc::vec;

use casper_contract::{
    contract_api::{runtime, account, storage},
    unwrap_or_revert::UnwrapOrRevert
};
use casper_types::{
    account::{
        AccountHash, Weight,
        AddKeyFailure, RemoveKeyFailure,
        UpdateKeyFailure
    },
    EntryPointAccess, EntryPointType, EntryPoint, EntryPoints,
    CLType, Parameter, CLTyped
};

mod errors;
use errors::Error;

const ARG_ACCOUNT_HASH: &str = "account_hash";
const ARG_WEIGHT: &str = "weight";

#[no_mangle]
pub extern "C" fn set_key_weight() {
    let account_hash: AccountHash = runtime::get_named_arg(ARG_ACCOUNT_HASH);
    let weight: Weight = Weight::new(runtime::get_named_arg(ARG_WEIGHT));
    update_key_weight(account_hash, weight);
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = get_entry_points();

    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("keys_manager_ext", contract_hash.into());
}

fn get_entry_points() -> EntryPoints {
    let remove_key = EntryPoint::new(
        String::from("set_key_weight"),
        vec![
            Parameter::new(ARG_ACCOUNT_HASH, AccountHash::cl_type()),
            Parameter::new(ARG_WEIGHT, CLType::U8)
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Session
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(remove_key);

    entry_points
}

fn update_key_weight(account: AccountHash, weight: Weight) {
    if weight.value() == 0 {
        remove_key_if_exists(account).unwrap_or_revert()
    } else {
        add_or_update_key(account, weight).unwrap_or_revert()
    }
}

fn remove_key_if_exists(key: AccountHash) -> Result<(), Error> {
    match account::remove_associated_key(key) {
        Ok(()) => Ok(()),
        Err(RemoveKeyFailure::MissingKey) => Ok(()),
        Err(RemoveKeyFailure::PermissionDenied) => Err(Error::PermissionDenied),
        Err(RemoveKeyFailure::ThresholdViolation) => Err(Error::ThresholdViolation),
    }
}

fn add_or_update_key(key: AccountHash, weight: Weight) -> Result<(), Error> {
    match account::update_associated_key(key, weight) {
        Ok(()) => Ok(()),
        Err(UpdateKeyFailure::MissingKey) => add_key(key, weight),
        Err(UpdateKeyFailure::PermissionDenied) => Err(Error::PermissionDenied),
        Err(UpdateKeyFailure::ThresholdViolation) => Err(Error::ThresholdViolation),
    }
}

fn add_key(key: AccountHash, weight: Weight) -> Result<(), Error> {
    match account::add_associated_key(key, weight) {
        Ok(()) => Ok(()),
        Err(AddKeyFailure::MaxKeysLimit) => Err(Error::MaxKeysLimit),
        Err(AddKeyFailure::DuplicateKey) => Err(Error::DuplicateKey), // Should never happen.
        Err(AddKeyFailure::PermissionDenied) => Err(Error::PermissionDenied),
    }
}
