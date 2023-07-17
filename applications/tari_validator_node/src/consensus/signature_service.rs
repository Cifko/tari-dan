//    Copyright 2023 The Tari Project
//    SPDX-License-Identifier: BSD-3-Clause

use std::sync::Arc;

use tari_common_types::types::PublicKey;
use tari_comms::NodeIdentity;
use tari_consensus::traits::{ValidatorSignatureService, VoteSignatureService};
use tari_dan_storage::consensus_models::ValidatorSchnorrSignature;

#[derive(Debug, Clone)]
pub struct TariSignatureService {
    node_identity: Arc<NodeIdentity>,
}

impl TariSignatureService {
    pub fn new(node_identity: Arc<NodeIdentity>) -> Self {
        Self { node_identity }
    }
}

impl ValidatorSignatureService for TariSignatureService {
    fn sign<M: AsRef<[u8]>>(&self, message: M) -> ValidatorSchnorrSignature {
        ValidatorSchnorrSignature::sign_message(self.node_identity.secret_key(), message).unwrap()
    }

    fn public_key(&self) -> &PublicKey {
        self.node_identity.public_key()
    }
}

impl VoteSignatureService for TariSignatureService {}
