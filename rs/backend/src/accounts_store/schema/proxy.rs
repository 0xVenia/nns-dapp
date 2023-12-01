//! Accounts DB that delegates API calls to underlying implementations.
//!
//! The proxy manages migrations from one implementation to another.
use super::{map::AccountsDbAsMap, Account, AccountsDbBTreeMapTrait, AccountsDbTrait, SchemaLabel};
use crate::accounts_store::schema::accounts_in_unbounded_stable_btree_map::AccountsDbAsUnboundedStableBTreeMap;
use ic_stable_structures::{memory_manager::VirtualMemory, DefaultMemoryImpl};
use std::collections::BTreeMap;
mod enum_boilerplate;

/// An accounts database delegates API calls to underlying implementations.
///
/// Notes:
/// - The proxy manages migrations from one implementation to another, if applicable.
/// - The proxy code itself will specify which databases are currently in
///   use and how to migrate from one database to another.
/// - It is the responsibility of the post-install hook to look at any
///   version information and set up the db accordingly.
///
/// # Current data storage
/// - Accounts are stored as a map.  No migrations are undertaken.
#[derive(Debug)]
pub struct AccountsDbAsProxy {
    authoritative_db: AccountsDb,
    second_db: Option<AccountsDb>,
}

#[derive(Debug)]

pub enum AccountsDb {
    Map(AccountsDbAsMap),
    UnboundedStableBTreeMap(AccountsDbAsUnboundedStableBTreeMap),
}

impl Default for AccountsDbAsProxy {
    fn default() -> Self {
        Self::new_with_map()
    }
}

// Constructors
impl AccountsDbAsProxy {
    pub fn new_with_map() -> Self {
        Self {
            authoritative_db: AccountsDb::Map(AccountsDbAsMap::default()),
            second_db: None,
        }
    }
    pub fn new_with_unbounded_stable_btree_map(memory: VirtualMemory<DefaultMemoryImpl>) -> Self {
        dfn_core::api::print("New Proxy: AccountsInStableMemory");
        Self {
            authoritative_db: AccountsDb::UnboundedStableBTreeMap(AccountsDbAsUnboundedStableBTreeMap::new(memory)),
            second_db: None,
        }
    }
}

impl AccountsDbAsProxy {
    /// Migration countdown; when it reaches zero, the migration is complete.
    pub fn migration_countdown(&self) -> u32 {
        0
    }
}

impl AccountsDbBTreeMapTrait for AccountsDbAsProxy {
    fn from_map(map: BTreeMap<Vec<u8>, Account>) -> Self {
        Self {
            authoritative_db: AccountsDb::Map(AccountsDbAsMap::from_map(map)),
            second_db: None,
        }
    }
    fn as_map(&self) -> BTreeMap<Vec<u8>, Account> {
        match &self.authoritative_db {
            AccountsDb::Map(map_db) => map_db.as_map(),
            AccountsDb::UnboundedStableBTreeMap(unbounded_stable_btree_map_db) => {
                unbounded_stable_btree_map_db.as_map()
            }
        }
    }
}

impl AccountsDbTrait for AccountsDbAsProxy {
    // Inserts into all the underlying databases.
    fn db_insert_account(&mut self, account_key: &[u8], account: Account) {
        self.authoritative_db.db_insert_account(account_key, account.clone());
        if let Some(db) = &mut self.second_db {
            db.db_insert_account(account_key, account);
        }
    }
    // Checks the authoritative database.
    fn db_contains_account(&self, account_key: &[u8]) -> bool {
        self.authoritative_db.db_contains_account(account_key)
    }
    // Gets an account from the authoritative database.
    fn db_get_account(&self, account_key: &[u8]) -> Option<Account> {
        self.authoritative_db.db_get_account(account_key)
    }
    // Removes an account from all underlying databases.
    fn db_remove_account(&mut self, account_key: &[u8]) {
        self.authoritative_db.db_remove_account(account_key);
        if let Some(db) = self.second_db.as_mut() {
            db.db_remove_account(account_key);
        }
    }
    // Gets the length from the authoritative database.
    fn db_accounts_len(&self) -> u64 {
        self.authoritative_db.db_accounts_len()
    }
    // Iterates over the entries of the authoritative database.
    fn iter(&self) -> Box<dyn Iterator<Item = (Vec<u8>, Account)> + '_> {
        self.authoritative_db.iter()
    }
    // Iterates over the values of the authoritative database.
    fn values(&self) -> Box<dyn Iterator<Item = Account> + '_> {
        self.authoritative_db.values()
    }
    fn schema_label(&self) -> SchemaLabel {
        let schema_label = self.authoritative_db.schema_label();
        dfn_core::api::print(format!("Getting authoritative schema label: {schema_label:#?}"));
        schema_label
    }
}

/// Check whether two account databases contain the same data.
///
/// It should be possible to use this to confirm that data has been preserved during a migration.
///
/// TODO: This is needed in tests only.  Disable otherwise.
impl PartialEq for AccountsDbAsProxy {
    fn eq(&self, other: &Self) -> bool {
        self.authoritative_db.as_map() == other.authoritative_db.as_map()
    }
}
impl Eq for AccountsDbAsProxy {}

#[cfg(test)]
mod tests {
    use super::super::tests::{assert_map_conversions_work, test_accounts_db};
    use super::AccountsDbAsProxy;

    test_accounts_db!(AccountsDbAsProxy::default());

    #[test]
    fn map_conversions_should_work() {
        assert_map_conversions_work::<AccountsDbAsProxy>();
    }
}
