mod collections;
mod helpers;
mod processes;

use std::time::Duration;

use crate::collections::{BaseNodes, BaseWallets, Collection};

// use std::ops::Sub;

// use reqwest::{
//     header::{self, HeaderMap, HeaderValue, CONTENT_TYPE},
//     Client,
// };
// use serde::{de::DeserializeOwned, Serialize};
// use serde_json::json;
// use tari_common_types::types::PublicKey;
// use tari_dan_wallet_sdk::models::SubstateType;
// use tari_engine_types::{instruction::Instruction, substate::SubstateId, TemplateAddress};
// use tari_template_builtin::{ACCOUNT_NFT_TEMPLATE_ADDRESS, ACCOUNT_TEMPLATE_ADDRESS};
// use tari_template_lib::{
//     args,
//     crypto::RistrettoPublicKeyBytes,
//     models::{Metadata, NonFungibleAddress},
//     prelude::NonFungibleId,
// };
// use tari_transaction::SubstateRequirement;
// use tari_utilities::byte_array::ByteArray;
// use tari_validator_node_client::types::{
//     GetTemplateRequest,
//     GetTemplateResponse,
//     GetTemplatesRequest,
//     GetTemplatesResponse,
// };
// use tari_wallet_daemon_client::{
//     types::{
//         AccountsCreateRequest,
//         AccountsCreateResponse,
//         AccountsListRequest,
//         AccountsListResponse,
//         AuthLoginAcceptRequest,
//         AuthLoginAcceptResponse,
//         AuthLoginRequest,
//         AuthLoginResponse,
//         CallInstructionRequest,
//         KeysCreateRequest,
//         KeysCreateResponse,
//         KeysListRequest,
//         KeysListResponse,
//         SubstatesListRequest,
//         SubstatesListResponse,
//         TransactionSubmitResponse,
//     },
//     ComponentAddressOrName,
// };

// async fn jrpc_call<I: Serialize, O: DeserializeOwned>(
//     url: &String,
//     method: &str,
//     params: I,
//     token: Option<String>,
// ) -> anyhow::Result<O> {
//     let client = Client::new();
//     let mut headers = HeaderMap::new();
//     headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
//     if let Some(token) = token {
//         headers.insert(
//             header::AUTHORIZATION,
//             HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
//         );
//     }
//     let res = client
//         .post(url)
//         .json(&json!({
//             "jsonrpc": "2.0",
//             "method": method,
//             "params": params,
//             "id": 1
//         }))
//         .headers(headers)
//         .send()
//         .await
//         .unwrap();
//     let json = res.json::<serde_json::Value>().await.unwrap();
//     let res = json.get("result").unwrap().clone();
//     Ok(serde_json::from_value::<O>(res)?)
// }

// pub struct DanJrpcClient {
//     url: String,
//     pub token: String,
// }

// pub struct VNJrpcClient {
//     url: String,
// }

// impl DanJrpcClient {
//     pub async fn new(url: &str) -> Self {
//         let request = AuthLoginRequest {
//             permissions: vec!["Admin".to_string()],
//             duration: None,
//         };
//         let resp: AuthLoginResponse = jrpc_call(&url.to_string(), "auth.request", request, None)
//             .await
//             .unwrap();
//         let auth_token = resp.auth_token;
//         let request = AuthLoginAcceptRequest {
//             auth_token,
//             name: "AdminToken".to_string(),
//         };
//         let resp: AuthLoginAcceptResponse = jrpc_call(&url.to_string(), "auth.accept", request, None).await.unwrap();
//         let token = resp.permissions_token;

//         DanJrpcClient {
//             url: url.to_string(),
//             token,
//         }
//     }

//     pub async fn accounts_create(&self) -> anyhow::Result<AccountsCreateResponse> {
//         let request = AccountsCreateRequest {
//             account_name: Some("Cifko".to_string()),
//             custom_access_rules: None,
//             max_fee: None,
//             is_default: false,
//             key_id: None,
//         };
//         jrpc_call(&self.url, "accounts.create", request, Some(self.token.clone())).await
//     }

//     pub async fn accounts_list(&self, offset: u64, limit: u64) -> anyhow::Result<AccountsListResponse> {
//         let request = AccountsListRequest { offset, limit };
//         jrpc_call(&self.url, "accounts.list", request, Some(self.token.clone())).await
//     }

//     pub async fn transaction_submit_instruction(
//         &self,
//         instructions: Vec<Instruction>,
//         fee_account: ComponentAddressOrName,
//         inputs: Vec<SubstateRequirement>,
//         inputs_refs: Vec<SubstateRequirement>,
//     ) -> anyhow::Result<TransactionSubmitResponse> {
//         let request = CallInstructionRequest {
//             instructions,
//             fee_account,
//             dump_outputs_into: None,
//             max_fee: 1000,
//             inputs,
//             override_inputs: Some(true),
//             new_outputs: None,
//             is_dry_run: false,
//             proof_ids: vec![],
//             min_epoch: None,
//             max_epoch: None,
//             inputs_refs,
//         };
//         jrpc_call(
//             &self.url,
//             "transactions.submit_instruction",
//             request,
//             Some(self.token.clone()),
//         )
//         .await
//     }

//     pub async fn keys_create(&self, specific_index: Option<u64>) -> KeysCreateResponse {
//         let request = KeysCreateRequest { specific_index };
//         jrpc_call(&self.url, "keys.create", request, Some(self.token.clone()))
//             .await
//             .unwrap()
//     }

//     pub async fn keys_list(&self) -> KeysListResponse {
//         let request = KeysListRequest {};
//         jrpc_call(&self.url, "keys.list", request, Some(self.token.clone()))
//             .await
//             .unwrap()
//     }

//     pub async fn substates_list(
//         &self,
//         filter_by_template: Option<TemplateAddress>,
//         filter_by_type: Option<SubstateType>,
//     ) -> SubstatesListResponse {
//         let request = SubstatesListRequest {
//             filter_by_type,
//             filter_by_template,
//         };
//         jrpc_call(&self.url, "substates.list", request, Some(self.token.clone()))
//             .await
//             .unwrap()
//     }
// }

// impl VNJrpcClient {
//     pub fn new(url: &str) -> Self {
//         VNJrpcClient { url: url.to_string() }
//     }

//     pub async fn get_template(&self, template_address: TemplateAddress) -> anyhow::Result<GetTemplateResponse> {
//         let request = GetTemplateRequest { template_address };
//         jrpc_call(&self.url, "get_template", request, None).await
//     }

//     pub async fn get_templates(&self) -> anyhow::Result<GetTemplatesResponse> {
//         let request = GetTemplatesRequest { limit: 20 };
//         jrpc_call(&self.url, "get_templates", request, None).await
//     }
// }

// async fn create_nft_account() {
//     let dan_jrpc = DanJrpcClient::new("http://localhost:18015").await;
//     // let key = dan_jrpc.keys_create(None).await;
//     // println!("Key Index {}", key.id);
//     let accounts = dan_jrpc.accounts_list(0, 10).await.unwrap();
//     let account = accounts.accounts[0].account.clone();
//     println!("Account {}", account.key_index);
//     let keys = dan_jrpc.keys_list().await;
//     let owner_pk = keys
//         .keys
//         .into_iter()
//         .find_map(|(id, key, _)| if id == account.key_index { Some(key) } else { None })
//         .unwrap();
//     // let owner_pk = &key.public_key;
//     let owner_token =
//         NonFungibleAddress::from_public_key(RistrettoPublicKeyBytes::from_bytes(owner_pk.as_bytes()).unwrap());

//     let instruction = Instruction::CallFunction {
//         template_address: ACCOUNT_NFT_TEMPLATE_ADDRESS,
//         function: "create".to_string(),
//         args: args![owner_token],
//     };
//     let resp = dan_jrpc
//         .transaction_submit_instruction(
//             vec![instruction],
//             ComponentAddressOrName::ComponentAddress(account.address.as_component_address().unwrap()),
//             vec![SubstateRequirement {
//                 substate_id: account.address,
//                 version: None,
//             }],
//             vec![],
//         )
//         .await
//         .unwrap();
//     println!("Response {:?}", resp);
// }

// async fn mint_nft() {
//     let dan_jrpc = DanJrpcClient::new("http://localhost:18015").await;
//     let accounts = dan_jrpc.accounts_list(0, 10).await.unwrap();
//     let fee_account = accounts.accounts.first().unwrap().account.clone();
//     println!("Fee account {}", fee_account.address);
//     let resp = dan_jrpc.substates_list(None, Some(SubstateType::Component)).await;
//     let component = resp
//         .substates
//         .iter()
//         .find(|component| component.module_name == Some("AccountNonFungible".to_string()))
//         .unwrap();
//     println!("Component {}", component.substate_id);
//     let resp = dan_jrpc.substates_list(None, Some(SubstateType::Resource)).await;
//     let resource = resp
//         .substates
//         .into_iter()
//         .find(|resource| resource.parent_id == Some(component.substate_id.clone()))
//         .unwrap();
//     println!("Resource {}", resource.substate_id);
//     let instructions = vec![
//         Instruction::CallMethod {
//             component_address: component.substate_id.as_component_address().unwrap(),
//             method: "mint".to_string(),
//             args: args![Metadata::new().insert("name","Cifko").insert("image_url", "https://archive.smashing.media/assets/344dbf88-fdf9-42bb-adb4-46f01eedd629/17ef2bdf-4d11-4fe1-9c44-cfe71e6202f2/icon-design-01-opt.png")],
//         },
//         Instruction::PutLastInstructionOutputOnWorkspace {
//             key: b"out_bucket".to_vec(),
//         },
//         Instruction::CallMethod {
//             component_address: fee_account.address.as_component_address().unwrap(),
//             method: "deposit".to_string(),
//             args: args![Variable("out_bucket")],
//         },
//     ];
//     let resp = dan_jrpc
//         .transaction_submit_instruction(
//             instructions,
//             ComponentAddressOrName::ComponentAddress(fee_account.address.as_component_address().unwrap()),
//             vec![
//                 SubstateRequirement {
//                     substate_id: fee_account.address,
//                     version: None,
//                 },
//                 SubstateRequirement {
//                     substate_id: component.substate_id.clone(),
//                     version: None,
//                 },
//             ],
//             vec![SubstateRequirement {
//                 substate_id: resource.substate_id.clone(),
//                 version: None,
//             }],
//         )
//         .await
//         .unwrap();
//     println!("Response {:?}", resp);
// }

// // async fn test_nft() {
// //     let vn_jrpc = VNJrpcClient::new("http://localhost:18018");
// //     let templates = vn_jrpc.get_templates().await.unwrap();
// //     let template = templates
// //         .templates
// //         .iter()
// //         .find(|template| template.name == "basic_nft.wasm")
// //         .unwrap();
// //     // println!("{:?}", template);
// //     let dan_jrpc = DanJrpcClient::new("http://localhost:18015").await;
// //     let accounts = dan_jrpc.accounts_list(0, 10).await.unwrap();
// //     let fee_account = accounts.accounts.first().unwrap().account.clone();
// //     let instruction = Instruction::CallFunction {
// //         template_address: template.address,
// //         function: "new_with_initial_nft".to_string(),
// //         args: args![NonFungibleId::String("1000".to_string())],
// //     };
// //     let resp = dan_jrpc
// //         .transaction_submit_instruction(
// //             vec![instruction],
// //             ComponentAddressOrName::ComponentAddress(fee_account.address.as_component_address().unwrap()),
// //             vec![SubstateRequirement {
// //                 substate_id: fee_account.address,
// //                 version: None,
// //             }],
// //             vec![],
// //             // vec![SubstateRequirement {
// //             //     substate_id: account.address,
// //             //     version: None,
// //             // }],
// //         )
// //         .await
// //         .unwrap();
// //     println!("Response {:?}", resp);
// //     // let account = accounts.accounts.last().unwrap().account.clone();

// //     // let ADDRESS = "e0aa15851c3158056f7ee180cad58519e78de1d26cc7c134bbf0acd0940bf5a5";
// // }

// // async fn test_mint() {
// //     let vn_jrpc = VNJrpcClient::new("http://localhost:18018");
// //     let templates = vn_jrpc.get_templates().await.unwrap();
// //     let template = templates
// //         .templates
// //         .iter()
// //         .find(|template| template.name == "basic_nft.wasm")
// //         .unwrap();
// //     let dan_jrpc = DanJrpcClient::new("http://localhost:18015").await;
// //     let accounts = dan_jrpc.accounts_list(0, 10).await.unwrap();
// //     let fee_account = accounts.accounts.first().unwrap().account.clone();
// //     let resp = dan_jrpc.substates_list(None, Some(SubstateType::Component)).await;
// //     let component = resp
// //         .substates
// //         .iter()
// //         .find(|component| component.template_address == Some(template.address))
// //         .unwrap();
// //     let resp = dan_jrpc.substates_list(None, Some(SubstateType::Resource)).await;
// //     let resource = resp
// //         .substates
// //         .into_iter()
// //         .find(|resource| resource.parent_id == Some(component.substate_id.clone()))
// //         .unwrap();
// //     let resp = dan_jrpc.substates_list(None, Some(SubstateType::Vault)).await;
// //     let vault = resp
// //         .substates
// //         .into_iter()
// //         .find(|vault| vault.parent_id == Some(component.substate_id.clone()))
// //         .unwrap();
// //     let instructions = vec![
// //         Instruction::CallMethod {
// //             component_address: component.substate_id.as_component_address().unwrap(),
// //             method: "mint".to_string(),
// //             args: args!["Cifko".to_string(), "https://cifko.com".to_string()],
// //         },
// //         Instruction::PutLastInstructionOutputOnWorkspace {
// //             key: b"out_bucket".to_vec(),
// //         },
// //         Instruction::CallMethod {
// //             component_address: fee_account.address.as_component_address().unwrap(),
// //             method: "deposit".to_string(),
// //             args: args![Variable("out_bucket")],
// //         },
// //     ];
// //     let resp = dan_jrpc
// //         .transaction_submit_instruction(
// //             instructions,
// //             ComponentAddressOrName::ComponentAddress(fee_account.address.as_component_address().unwrap()),
// //             vec![
// //                 SubstateRequirement {
// //                     substate_id: fee_account.address,
// //                     version: None,
// //                 },
// //                 SubstateRequirement {
// //                     substate_id: component.substate_id.clone(),
// //                     version: None,
// //                 },
// //                 SubstateRequirement {
// //                     substate_id: vault.substate_id.clone(),
// //                     version: None,
// //                 },
// //             ],
// //             // vec![],
// //             vec![SubstateRequirement {
// //                 substate_id: resource.substate_id.clone(),
// //                 version: None,
// //             }],
// //         )
// //         .await
// //         .unwrap();
// //     println!("Response {:?}", resp);
// // }

#[tokio::main]
async fn main() {
    // create_nft_account().await;
    // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    // mint_nft().await;
    let mut base_nodes = BaseNodes::new();
    let mut base_wallets = BaseWallets::new();
    base_nodes.add();
    println!("{:?}", base_nodes.get_addresses());
    let version = base_nodes
        .get_any_mut()
        .unwrap()
        .get_client()
        .await
        .get_version(tonic::Request::new(minotari_node_grpc_client::grpc::Empty {}))
        .await
        .unwrap()
        .into_inner();
    println!("{}", version.value);
    base_wallets.add(base_nodes.get_any().unwrap().get_address(), base_nodes.get_addresses());
    let version = base_wallets
        .get_any_mut()
        .unwrap()
        .get_client()
        .await
        .get_version(tonic::Request::new(
            minotari_wallet_grpc_client::grpc::GetVersionRequest {},
        ))
        .await
        .unwrap()
        .into_inner();
    println!("{}", version.version);

    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("End");
}
