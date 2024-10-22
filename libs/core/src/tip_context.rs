use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use spectre_wallet_core::{rpc::RpcApi, storage::Interface, wallet::Wallet};
use spectre_wrpc_client::{prelude::NetworkId, Resolver, SpectreRpcClient};

use crate::{result::Result, tip_owned_wallet::TipOwnedWallet};

pub struct TipContext {
    resolver: Resolver,
    network_id: NetworkId,
    opened_wallet: RwLock<HashMap<String, TipOwnedWallet>>,
    forced_node_url: Option<String>,
    wrpc_client: Arc<SpectreRpcClient>,
}

impl TipContext {
    pub fn new_arc(
        resolver: Resolver,
        network_id: NetworkId,
        forced_node_url: Option<String>,
        wrpc_client: Arc<SpectreRpcClient>,
    ) -> Arc<Self> {
        Arc::new(TipContext {
            network_id,
            resolver,
            forced_node_url,
            wrpc_client,
            opened_wallet: RwLock::new(HashMap::new()),
        })
    }

    pub fn network_id(&self) -> NetworkId {
        self.network_id.clone()
    }

    pub fn resolver(&self) -> Resolver {
        self.resolver.clone()
    }

    pub fn get_opened_wallet_rw_lock(&self) -> &RwLock<HashMap<String, TipOwnedWallet>> {
        return &self.opened_wallet;
    }

    pub fn does_open_wallet_exists(&self, identifier: &str) -> bool {
        let read_lock = self.opened_wallet.read().unwrap();

        read_lock.contains_key(identifier)
    }

    /**
     * return a cloned version of the wallet, if found
     */
    pub fn get_open_wallet_arc(&self, identifier: &str) -> Option<TipOwnedWallet> {
        let read_lock = self.opened_wallet.read().unwrap();

        let wallet_result = read_lock.get(identifier);

        wallet_result.cloned()
    }

    pub fn add_opened_wallet(&self, identifier: String, wallet: TipOwnedWallet) -> TipOwnedWallet {
        let mut lock = self.opened_wallet.write().unwrap();
        lock.insert(identifier, wallet.clone());

        return wallet;
    }

    /*
     * closing the wallet has to be done externally
     */
    pub fn remove_opened_wallet(&self, identifier: &str) -> Option<TipOwnedWallet> {
        let mut lock = self.opened_wallet.write().unwrap();
        let tip_wallet_result = lock.remove(identifier);

        tip_wallet_result
    }

    /*
     * get a new store
     */
    pub fn local_store(&self) -> Result<Arc<dyn Interface>> {
        Ok(Wallet::local_store()?)
    }

    pub fn forced_node_url(&self) -> Option<String> {
        self.forced_node_url.clone()
    }

    pub fn rpc_api(&self) -> Arc<dyn RpcApi> {
        self.wrpc_client.clone()
    }
}
