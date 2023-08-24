//! Generic tests for account storage.

use super::super::{AccountIdentifier, CanisterId, NamedCanister, PrincipalId};
use super::*;
use std::collections::{BTreeMap, HashMap};

/// Toy accounts database for testing.
#[derive(Default)]
pub struct MockAccountsDb {
    accounts: BTreeMap<Vec<u8>, Account>,
}

impl AccountsDbTrait for MockAccountsDb {
    fn db_insert_account(&mut self, account_key: &[u8], account: Account) {
        self.accounts.insert(account_key.to_vec(), account);
    }
    fn db_contains_account(&self, account_key: &[u8]) -> bool {
        self.accounts.contains_key(account_key)
    }
    fn db_get_account(&self, account_key: &[u8]) -> Option<Account> {
        self.accounts.get(account_key).cloned()
    }
    fn db_remove_account(&mut self, account_key: &[u8]) {
        self.accounts.remove(account_key);
    }
    fn db_accounts_len(&self) -> u64 {
        self.accounts.len() as u64
    }
}

/// Creates a toy canister.
fn toy_canister(account_index: u64, canister_index: u64) -> NamedCanister {
    let canister_id = CanisterId::from(canister_index);
    NamedCanister {
        name: format!("canister_{account_index}_{canister_index}"),
        canister_id,
    }
}

/// Creates a toy account.  The contents do not need to be meaningful; do need to have size.
fn toy_account(account_index: u64, num_canisters: u64) -> Account {
    let principal = PrincipalId::new_user_test_id(account_index);
    let account_identifier = AccountIdentifier::from(principal);
    let mut account = Account {
        principal: Some(principal),
        account_identifier,
        default_account_transactions: Vec::new(),
        sub_accounts: HashMap::new(),
        hardware_wallet_accounts: Vec::new(),
        canisters: Vec::new(),
    };
    // Attaches canisters to the account.
    for canister_index in 0..num_canisters {
        account.canisters.push(toy_canister(account_index, canister_index));
    }
    // FIN
    account
}

/// Verifies that an arbitrary AccountsDbTrait implementation does basic crud correctly.
///
/// Individual implementations are expected to perform their own tests for error conditions
/// relevant to them.
fn assert_basic_crud_works<D>(mut storage: D)
where
    D: AccountsDbTrait,
{
    let account_key = vec![1, 2, 3];
    let account = toy_account(1, 5);
    // Create:
    storage.db_insert_account(&account_key, account.clone());
    // Read:
    assert!(storage.db_contains_account(&account_key));
    assert_eq!(storage.db_get_account(&account_key), Some(account.clone()));
    // Update:
    let updated_account = toy_account(1, 1000);
    storage.db_insert_account(&account_key, updated_account.clone());
    assert!(storage.db_contains_account(&account_key));
    assert_eq!(storage.db_get_account(&account_key), Some(updated_account.clone()));
    // Delete:
    storage.db_remove_account(&account_key);
    assert!(!storage.db_contains_account(&account_key));
    assert_eq!(storage.db_get_account(&account_key), None);
}

/// Verifies that the update function `db_try_with_account()` works correctly.
fn assert_update_with_happy_path_works<D>(mut storage: D)
where
    D: AccountsDbTrait,
{
    let account_key = vec![1, 2, 3];
    let account = toy_account(1, 5);
    // Create:
    storage.db_insert_account(&account_key, account.clone());
    assert_eq!(storage.db_get_account(&account_key), Some(account.clone()));
    // Update:
    // Modify by adding a canister
    {
        // We will add a new canister:
        let canister = toy_canister(3, 14159265359);
        let expected_last_canister = canister.clone();
        // Verify that it is not already the last...
        assert_ne!(
            storage
                .db_get_account(&account_key)
                .expect("Failed to get account")
                .canisters
                .last()
                .expect("Account should have had canisters"),
            &expected_last_canister
        );
        // The function return value; it should be passed through.
        let return_value: Result<i32, i32> = Ok(42);
        // Update the account:
        let actual_return_value = storage.db_try_with_account(&account_key, move |account| {
            account.canisters.push(canister.clone());
            // The return value should be passed through.
            return_value
        });
        assert_eq!(Some(return_value), actual_return_value);
        // Verify that the new canister is now the last.
        assert_eq!(
            storage
                .db_get_account(&account_key)
                .expect("Failed to get account")
                .canisters
                .last()
                .expect("Account should have had canisters"),
            &expected_last_canister
        );
    }
}

/// Verifies that the update function `db_try_with_account()` does NOT save changes if
/// the modifying function returns an error.
fn assert_update_not_saved_on_error<D>(mut storage: D)
where
    D: AccountsDbTrait,
{
    let account_key = vec![1, 2, 3];
    let account_initial_value = toy_account(1, 5);
    // Create:
    storage.db_insert_account(&account_key, account_initial_value.clone());
    assert_eq!(
        storage.db_get_account(&account_key),
        Some(account_initial_value.clone())
    );
    // Update:
    // Modify by adding a canister but then return an error.
    {
        // We will add a new canister:
        let canister = toy_canister(3, 14159265359);
        let expected_last_canister = canister.clone();
        // Verify that it is not already the last...
        assert_ne!(
            storage
                .db_get_account(&account_key)
                .expect("Failed to get account")
                .canisters
                .last()
                .expect("Account should have had canisters"),
            &expected_last_canister
        );
        // The function return value; it should be passed through.
        let return_value: Result<i32, i32> = Err(42);
        // Update the account:
        let actual_return_value = storage.db_try_with_account(&account_key, move |account| {
            account.canisters.push(canister.clone());
            // The return value should be passed through.
            return_value
        });
        assert_eq!(Some(return_value), actual_return_value);
        // Verify that the account has not changed.
        assert_eq!(
            storage.db_get_account(&account_key).expect("Failed to get account"),
            account_initial_value
        );
    }
}

/// Verifies that the update function `db_try_with_account()` returns None if there is no account
/// for the given key.
fn assert_update_with_missing_key_returns_none<D>(mut storage: D)
where
    D: AccountsDbTrait,
{
    let account_key = vec![1, 2, 3];
    let account_key_2 = vec![1, 2, 3, 4];
    let account = toy_account(1, 5);
    // Create:
    storage.db_insert_account(&account_key, account.clone());
    assert_eq!(storage.db_get_account(&account_key), Some(account.clone()));
    // Updates:
    // Modifies by adding a canister
    {
        // Updates the account:
        let actual_return_value: Option<Result<i32, i32>> = storage
            .db_try_with_account(&account_key_2, move |_account| {
                panic!("If the requested account is not found, the update function should not be called.")
            });
        assert_eq!(None, actual_return_value);
        // Verifies that the one account we created before the update call is unchanged.
        assert_eq!(
            storage.db_get_account(&account_key).expect("Failed to get account"),
            account
        );
    }
}

/// Verifies that the account count is correct.
fn assert_account_count_is_correct<D>(mut storage: D)
where
    D: AccountsDbTrait,
{
    // We will generate this many accounts in this test
    const NUM_TEST_ACCOUNTS: u64 = 10;

    // Loookup key for a test account
    fn account_key(account_index: u64) -> [u8; 1] {
        [account_index as u8 + 5]
    }

    // Local toy account, making sure that the index and canister ID are not the same
    fn test_account(account_index: u64) -> Account {
        let account_id = 100 - account_index;
        let num_canisters = 10 * account_index * account_index; // This takes up to 810 canisters, which should be enough to stress any storage backend.
        toy_account(account_id, num_canisters)
    }

    // Verify that the provided database is empty.
    assert_eq!(
        storage.db_accounts_len(),
        0,
        "The account database should be empty before we start testing."
    );
    for account_index in 0..NUM_TEST_ACCOUNTS {
        storage.db_insert_account(&account_key(account_index), test_account(account_index));
        assert_eq!(
            account_index + 1,
            storage.db_accounts_len(),
            "Number of canisters does not correspond to the number of canisters inserted."
        );
    }
    // Modifying accounts should not change the length.
    assert_eq!(
        NUM_TEST_ACCOUNTS,
        storage.db_accounts_len(),
        "Expected to have all the canisters by now."
    );
    let response: Result<(), ()> = Ok(());
    storage.db_try_with_account(&[0], move |_| response);
    assert_eq!(
        NUM_TEST_ACCOUNTS,
        storage.db_accounts_len(),
        "Modifying a canister should not change the count."
    );

    // Deleting accounts should reduce the length.
    // To test, we will delete the first, one in the middle and the last.
    storage.db_remove_account(&account_key(0));
    assert_eq!(
        NUM_TEST_ACCOUNTS - 1,
        storage.db_accounts_len(),
        "Deleting one account should reduce the account count by one."
    );
    storage.db_remove_account(&account_key(5));
    assert_eq!(
        NUM_TEST_ACCOUNTS - 2,
        storage.db_accounts_len(),
        "Deleting two accounts should reduce the account count by two."
    );
    storage.db_remove_account(&account_key(NUM_TEST_ACCOUNTS - 1));
    assert_eq!(
        NUM_TEST_ACCOUNTS - 3,
        storage.db_accounts_len(),
        "Deleting three accounts should reduce the account count by three."
    );
    // Deleting a non-existent account, or an account that has already been deleted, should change nothing.
    storage.db_remove_account(&account_key(NUM_TEST_ACCOUNTS - 1));
    assert_eq!(
        NUM_TEST_ACCOUNTS - 3,
        storage.db_accounts_len(),
        "Deleting an account again should not affect the count.."
    );
    storage.db_remove_account(&account_key(NUM_TEST_ACCOUNTS + 1));
    assert_eq!(
        NUM_TEST_ACCOUNTS - 3,
        storage.db_accounts_len(),
        "Deleting a non-existent canister should not affect the count."
    );
}

#[test]
fn mock_accounts_db_should_crud() {
    assert_basic_crud_works(MockAccountsDb::default());
}

#[test]
fn mock_accounts_update_with_happy_path_should_update_account() {
    assert_update_with_happy_path_works(MockAccountsDb::default());
}

#[test]
fn mock_accounts_update_with_error_path_should_not_change_account() {
    assert_update_not_saved_on_error(MockAccountsDb::default());
}

#[test]
fn mock_update_with_missing_key_should_return_none() {
    assert_update_with_missing_key_returns_none(MockAccountsDb::default());
}

#[test]
fn mock_account_counts_should_be_correct() {
    assert_account_count_is_correct(MockAccountsDb::default());
}
