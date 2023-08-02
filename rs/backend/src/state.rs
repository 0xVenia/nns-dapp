use crate::accounts_store::AccountsStore;
use crate::assets::AssetHashes;
use crate::assets::Assets;
use crate::perf::PerformanceCounts;
use dfn_candid::Candid;
use dfn_core::{api::trap_with, stable};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use on_wire::{FromWire, IntoWire};
use std::cell::RefCell;
#[cfg(test)]
pub mod tests;

#[derive(Default, Debug, Eq, PartialEq)]
pub struct State {
    // NOTE: When adding new persistent fields here, ensure that these fields
    // are being persisted in the `replace` method below.
    pub accounts_store: RefCell<AccountsStore>,
    pub assets: RefCell<Assets>,
    pub asset_hashes: RefCell<AssetHashes>,
    pub performance: RefCell<PerformanceCounts>,
}

impl State {
    pub fn replace(&self, new_state: State) {
        self.accounts_store.replace(new_state.accounts_store.take());
        self.assets.replace(new_state.assets.take());
        self.asset_hashes.replace(new_state.asset_hashes.take());
        self.performance.replace(new_state.performance.take());
    }
}

pub trait StableState: Sized {
    fn encode(&self) -> Vec<u8>;
    fn decode(bytes: Vec<u8>) -> Result<Self, String>;
}

// Stable memory is split into several virtual memories for different purposes.
type Memory = VirtualMemory<DefaultMemoryImpl>;
const CONTROL_MEMORY_ID: MemoryId = MemoryId::new(0);
const HEAP_MEMORY_ID: MemoryId = MemoryId::new(1);
const ACCOUNTS_DATA_MEMORY_ID_SCHEMA_A: MemoryId = MemoryId::new(2);
const ACCOUNTS_DATA_MEMORY_ID_SCHEMA_B: MemoryId = MemoryId::new(3);

thread_local! {
    pub static STATE: State = State::default();

    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize a `StableBTreeMap` that holds the accounts data.
    // TODO: Change the key to a struct consisting of pagenum, principal length and a byte vec.
    // TODO: Change the value to a 1kb page; u16len+data; use -1 if the page is full and there is a follow-on page.
    static ACCOUNTS_DATA_A: RefCell<StableBTreeMap<[u8;32], [u8;1024], Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

impl StableState for State {
    fn encode(&self) -> Vec<u8> {
        Candid((self.accounts_store.borrow().encode(), self.assets.borrow().encode()))
            .into_bytes()
            .unwrap()
    }

    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let (account_store_bytes, assets_bytes) = Candid::from_bytes(bytes).map(|c| c.0)?;

        let assets = Assets::decode(assets_bytes)?;
        let asset_hashes = AssetHashes::from(&assets);
        let performance = PerformanceCounts::default();

        Ok(State {
            accounts_store: RefCell::new(AccountsStore::decode(account_store_bytes)?),
            assets: RefCell::new(assets),
            asset_hashes: RefCell::new(asset_hashes),
            performance: RefCell::new(performance),
        })
    }
}

// Methods called on pre_upgrade and post_upgrade.
impl State {
    /// The schema version, determined by the last version that was saved to stable memory.
    fn schema_version_from_stable_memory() -> Option<u32> {
        None // The schema is currently unversioned.
    }
    /// Create the state from stable memory in the post_upgrade() hook.
    ///
    /// Note: The stable memory may have been created by any of these schemas:
    /// - The previous schema, when first migrating from the previous schema to the current schema.
    /// - The curent schema, if upgrading without changing the schema.
    /// - The next schema, if a new schema was deployed and we need to roll back.
    ///
    /// Note: Changing the schema requires at least two deployments:
    /// - Deploy a relase with a parser for the new schema.
    /// - Then, deploy a release that writes the new schema.
    /// This way it is possible to roll back after deploying the new schema.
    pub fn post_upgrade() -> Self {
        match Self::schema_version_from_stable_memory() {
            None => Self::post_upgrade_unversioned(),
            Some(version) => {
                trap_with(&format!("Unknown schema version: {version}"));
                unreachable!();
            }
        }
    }
    /// Save any unsaved state to stable memory.
    pub fn pre_upgrade(&self) {
        self.pre_upgrade_unversioned()
    }
}

// The unversionsed schema.
impl State {
    /// Save any unsaved state to stable memory.
    fn pre_upgrade_unversioned(&self) {
        let bytes = self.encode();
        stable::set(&bytes);
    }
    /// Create the state from stable memory in the post_upgrade() hook.
    fn post_upgrade_unversioned() -> Self {
        let bytes = stable::get();
        State::decode(bytes).unwrap_or_else(|e| {
            trap_with(&format!("Decoding stable memory failed. Error: {e:?}"));
            unreachable!();
        })
    }
}

// The S0 schema.
// * S0 stores accounts in a BTreeMap.
impl State {
    /// Migrate from unversioned.
    fn migrate_from_unversioned() {
        // TODO: Do in multiple steps.
        unimplemented!()
        // TODO: when done, flip the version.
    }
    /// Save any unsaved state to stable memory.
    fn pre_upgrade_s0(&self) {
        unimplemented!()
    }
    /// Create the state from stable memory in the post_upgrade() hook.
    fn post_upgrade_s0() -> Self {
        unimplemented!()
    }
}
