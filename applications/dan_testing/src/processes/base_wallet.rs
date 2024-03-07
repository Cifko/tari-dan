use minotari_wallet_grpc_client::WalletGrpcClient;

use super::process::Process;
use crate::helpers::{config::CONFIG, ports::get_port};

type GrpcClient = WalletGrpcClient<tonic::transport::Channel>;
pub struct BaseWallet {
    pub process: Process,
    pub public_address: String,
    pub grpc_port: u16,
    pub public_port: u16,
}

impl BaseWallet {
    pub async fn get_client(&self) -> GrpcClient {
        let address = format!("http://{}:{}", CONFIG.local_ip, self.grpc_port);
        WalletGrpcClient::connect(address.as_str()).await.unwrap()
    }

    pub fn new(name: &String, custom_base_node: String, peer_seeds: Vec<String>) -> Self {
        let path = CONFIG.minotari_path.join("minotari_console_wallet");
        let public_port = get_port();
        let public_address = format!("/ip4/{}/tcp/{}", CONFIG.local_ip, public_port);
        let grpc_port = get_port();
        let base_path = CONFIG.data_folder.join(name);
        let listener_address = format!("wallet.p2p.transport.tcp.listener_address={}", public_address);
        let public_addresses = format!("wallet.p2p.public_addresses={}", public_address);
        let grpc_address = format!("wallet.grpc_address=/ip4/{}/tcp/{}", CONFIG.local_ip, grpc_port);
        let custom_base_node = format!("wallet.custom_base_node={}", custom_base_node);
        let peer_addresses = format!("localnet.p2p.seeds.peer_seeds={}", peer_seeds.join(","));
        let args = vec![
            "-b",
            base_path.to_str().unwrap(),
            "-n",
            "--network",
            "localnet",
            "--enable-grpc",
            "--password",
            "a",
            "-p",
            "wallet.p2p.transport.type=tcp",
            "-p",
            listener_address.as_str(),
            "-p",
            public_addresses.as_str(),
            "-p",
            grpc_address.as_str(),
            "-p",
            custom_base_node.as_str(),
            "-p",
            "wallet.p2p.allow_test_addresses=true",
            "-p",
            peer_addresses.as_str(),
        ];
        let mut process = Process::new(name.clone(), path, args);
        process.run();
        BaseWallet {
            process,
            public_address: public_addresses,
            grpc_port,
            public_port,
        }
    }
}
