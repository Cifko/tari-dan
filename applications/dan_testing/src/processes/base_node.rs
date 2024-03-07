use std::{fs, thread, time};

use minotari_node_grpc_client::BaseNodeGrpcClient;
use tari_comms::NodeIdentity;

use super::process::Process;
use crate::helpers::{config::CONFIG, ports::get_port};

type GrpcClient = BaseNodeGrpcClient<tonic::transport::Channel>;

pub struct BaseNode {
    pub process: Process,
    pub public_address: String,
    pub grpc_port: u16,
    pub public_port: u16,
}

impl BaseNode {
    pub async fn get_client(&self) -> GrpcClient {
        let address = format!("http://{}:{}", CONFIG.local_ip, self.grpc_port);
        BaseNodeGrpcClient::connect(address).await.unwrap()
    }

    pub fn new(name: &String, peer_seeds: Vec<String>) -> Self {
        let command = CONFIG.minotari_path.join("minotari_node");
        println!("Path : {:?}", command);
        println!("Data folder : {:?}", CONFIG.data_folder.join(format!("{name}.log")));
        let public_port = get_port();
        let public_address = format!("/ip4/{}/tcp/{}", CONFIG.local_ip, public_port);
        let grpc_port = get_port();
        let base_path = CONFIG.data_folder.join(name);
        let listener_address = format!("base_node.p2p.transport.tcp.listener_address={}", public_address);
        let public_addresses = format!("base_node.p2p.public_addresses={}", public_address);
        let grpc_address = format!("base_node.grpc_address=/ip4/{}/tcp/{}", CONFIG.local_ip, grpc_port);
        let peer_addresses = format!("localnet.p2p.seeds.peer_seeds={}", peer_seeds.join(","));
        let mut process = Process::new(name.clone(), command, vec![
            "-b",
            base_path.to_str().unwrap(),
            "-n",
            "--network",
            "localnet",
            "--second-layer-grpc-enabled",
            "--mining-enabled",
            "-p",
            "base_node.p2p.transport.type=tcp",
            "-p",
            listener_address.as_str(),
            "-p",
            public_addresses.as_str(),
            "-p",
            grpc_address.as_str(),
            "-p",
            "base_node.grpc_enabled=true",
            "-p",
            "base_node.p2p.allow_test_addresses=true",
            "-p",
            "base_node.metadata_auto_ping_interval=3",
            "-p",
            "base_node.report_grpc_error=true",
            "-p",
            peer_addresses.as_str(),
        ]);
        process.run();
        BaseNode {
            process,
            public_address: public_addresses,
            grpc_port,
            public_port,
        }
    }

    pub fn get_address(&self) -> String {
        let base_node_id_file_path = CONFIG
            .data_folder
            .join(&self.process.name)
            .join("localnet")
            .join("config")
            .join("base_node_id.json");
        while !std::path::Path::new(&base_node_id_file_path).exists() {
            thread::sleep(time::Duration::from_secs(1));
        }
        let id_str = fs::read_to_string(base_node_id_file_path).unwrap();
        let id = json5::from_str::<NodeIdentity>(&id_str).unwrap();
        format!("{}::{}", id.public_key(), id.first_public_address().unwrap())
    }
}
