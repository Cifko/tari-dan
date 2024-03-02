use std::ops::Sub;

use reqwest::{
    header::{self, HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use tari_common_types::types::PublicKey;
use tari_engine_types::{instruction::Instruction, substate::SubstateId, TemplateAddress};
use tari_template_builtin::{ACCOUNT_NFT_TEMPLATE_ADDRESS, ACCOUNT_TEMPLATE_ADDRESS};
use tari_template_lib::{
    args,
    crypto::RistrettoPublicKeyBytes,
    models::{Metadata, NonFungibleAddress},
};
use tari_transaction::SubstateRequirement;
use tari_utilities::byte_array::ByteArray;
use tari_validator_node_client::types::{
    GetTemplateRequest,
    GetTemplateResponse,
    GetTemplatesRequest,
    GetTemplatesResponse,
};
use tari_wallet_daemon_client::{
    types::{
        AccountsCreateRequest,
        AccountsCreateResponse,
        AccountsListRequest,
        AccountsListResponse,
        AuthLoginAcceptRequest,
        AuthLoginAcceptResponse,
        AuthLoginRequest,
        AuthLoginResponse,
        CallInstructionRequest,
        KeysCreateRequest,
        KeysCreateResponse,
        TransactionSubmitResponse,
    },
    ComponentAddressOrName,
};

async fn jrpc_call<I: Serialize, O: DeserializeOwned>(
    url: &String,
    method: &str,
    params: I,
    token: Option<String>,
) -> anyhow::Result<O> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(token) = token {
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }
    let res = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        }))
        .headers(headers)
        .send()
        .await
        .unwrap();
    let json = res.json::<serde_json::Value>().await.unwrap();
    println!("---------------");
    println!("{:?}", json);
    let res = json.get("result").unwrap().clone();
    Ok(serde_json::from_value::<O>(res)?)
}

pub struct DanJrpcClient {
    url: String,
    pub token: String,
}

pub struct VNJrpcClient {
    url: String,
}

impl DanJrpcClient {
    pub async fn new(url: &str) -> Self {
        let request = AuthLoginRequest {
            permissions: vec!["Admin".to_string()],
            duration: None,
        };
        let resp: AuthLoginResponse = jrpc_call(&url.to_string(), "auth.request", request, None)
            .await
            .unwrap();
        let auth_token = resp.auth_token;
        let request = AuthLoginAcceptRequest {
            auth_token,
            name: "AdminToken".to_string(),
        };
        let resp: AuthLoginAcceptResponse = jrpc_call(&url.to_string(), "auth.accept", request, None).await.unwrap();
        let token = resp.permissions_token;

        DanJrpcClient {
            url: url.to_string(),
            token,
        }
    }

    pub async fn accounts_create(&self) -> anyhow::Result<AccountsCreateResponse> {
        let request = AccountsCreateRequest {
            account_name: Some("Cifko".to_string()),
            custom_access_rules: None,
            max_fee: None,
            is_default: false,
            key_id: None,
        };
        jrpc_call(&self.url, "accounts.create", request, Some(self.token.clone())).await
    }

    pub async fn accounts_list(&self, offset: u64, limit: u64) -> anyhow::Result<AccountsListResponse> {
        let request = AccountsListRequest { offset, limit };
        jrpc_call(&self.url, "accounts.list", request, Some(self.token.clone())).await
    }

    pub async fn transaction_submit_instruction(
        &self,
        instruction: Instruction,
        fee_account: ComponentAddressOrName,
        inputs: Vec<SubstateRequirement>,
        inputs_refs: Vec<SubstateRequirement>,
    ) -> anyhow::Result<TransactionSubmitResponse> {
        let request = CallInstructionRequest {
            instructions: vec![instruction],
            fee_account,
            dump_outputs_into: None,
            max_fee: 1000,
            inputs,
            override_inputs: Some(true),
            new_outputs: None,
            is_dry_run: false,
            proof_ids: vec![],
            min_epoch: None,
            max_epoch: None,
            inputs_refs,
        };
        jrpc_call(
            &self.url,
            "transactions.submit_instruction",
            request,
            Some(self.token.clone()),
        )
        .await
    }

    pub async fn keys_create(&self, specific_index: Option<u64>) -> KeysCreateResponse {
        let request = KeysCreateRequest { specific_index };
        jrpc_call(&self.url, "keys.create", request, Some(self.token.clone()))
            .await
            .unwrap()
    }
}

impl VNJrpcClient {
    pub fn new(url: &str) -> Self {
        VNJrpcClient { url: url.to_string() }
    }

    pub async fn get_template(&self, template_address: TemplateAddress) -> anyhow::Result<GetTemplateResponse> {
        let request = GetTemplateRequest { template_address };
        jrpc_call(&self.url, "get_template", request, None).await
    }

    pub async fn get_templates(&self) -> anyhow::Result<GetTemplatesResponse> {
        let request = GetTemplatesRequest { limit: 0 };
        jrpc_call(&self.url, "get_templates", request, None).await
    }
}

async fn create_nft_account() {
    let dan_jrpc = DanJrpcClient::new("http://localhost:18015").await;
    let key = dan_jrpc.keys_create(None).await;
    println!("Key Index {}", key.id);
    let accounts = dan_jrpc.accounts_list(0, 10).await.unwrap();
    let account = accounts.accounts[0].account.clone();

    let owner_pk = &key.public_key;
    let owner_token =
        NonFungibleAddress::from_public_key(RistrettoPublicKeyBytes::from_bytes(owner_pk.as_bytes()).unwrap());

    let instruction = Instruction::CallFunction {
        template_address: ACCOUNT_NFT_TEMPLATE_ADDRESS,
        function: "create".to_string(),
        args: args![owner_token],
    };
    let resp = dan_jrpc
        .transaction_submit_instruction(
            instruction,
            ComponentAddressOrName::ComponentAddress(account.address.as_component_address().unwrap()),
            vec![SubstateRequirement {
                substate_id: account.address,
                version: None,
            }],
            vec![],
        )
        .await
        .unwrap();
    println!("Response {:?}", resp);
}

async fn mint_nft() {
    let dan_jrpc = DanJrpcClient::new("http://localhost:18015").await;
    let accounts = dan_jrpc.accounts_list(0, 10).await.unwrap();
    let fee_account = accounts.accounts.first().unwrap().account.clone();
    let account = accounts.accounts.last().unwrap().account.clone();

    let instruction = Instruction::CallMethod {
        component_address: account.address.as_component_address().unwrap(),
        method: "mint".to_string(),
        args: args![Metadata::new().insert("name", "Cifko")],
    };
    let resp = dan_jrpc
        .transaction_submit_instruction(
            instruction,
            ComponentAddressOrName::ComponentAddress(fee_account.address.as_component_address().unwrap()),
            vec![SubstateRequirement {
                substate_id: account.address,
                version: None,
            }],
            vec![SubstateRequirement {
                substate_id: fee_account.address,
                version: None,
            }],
        )
        .await
        .unwrap();
    println!("Response {:?}", resp);
}

#[tokio::main]
async fn main() {
    // create_nft_account().await;
    mint_nft().await;
}
