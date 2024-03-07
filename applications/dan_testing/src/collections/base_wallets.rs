use std::collections::HashMap;

use super::collection::Collection;
use crate::processes::base_wallet::BaseWallet;

pub struct BaseWallets {
    wallets: HashMap<String, BaseWallet>,
}

impl Collection<BaseWallet> for BaseWallets {
    fn new() -> Self {
        BaseWallets {
            wallets: HashMap::new(),
        }
    }

    fn items(&self) -> &HashMap<String, BaseWallet> {
        return &self.wallets;
    }

    fn items_mut(&mut self) -> &mut HashMap<String, BaseWallet> {
        return &mut self.wallets;
    }
}

impl BaseWallets {
    pub fn add(&mut self, custom_base_node: String, peer_seeds: Vec<String>) {
        let new_name = format!("BaseWallet{}", self.wallets.len());
        let base_wallet = BaseWallet::new(&new_name, custom_base_node, peer_seeds);
        self.wallets.insert(new_name, base_wallet);
    }
}
