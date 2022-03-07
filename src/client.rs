use tokio::runtime::{Builder, Runtime};
use crate::chain_broker_client::ChainBrokerClient;
use crate::pb::abi::*;


type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;

struct BlockingClient {
    client: ChainBrokerClient<tonic::transport::Channel>,
    rt: Runtime,
}

impl BlockingClient {
    pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        let client = rt.block_on(ChainBrokerClient::connect(dst))?;
        Ok(Self { client, rt })
    }

    pub fn get_block(
        &mut self,
        typ: i32,
        value: &str,
    ) -> Result<tonic::Response<Block>, tonic::Status> {
        let request = tonic::Request::new(GetBlockRequest {
            r#type: typ,
            value: value.into(),
        });
        self.rt.block_on(self.client.get_block(request))
    }

    pub fn get_block_headers(
        &mut self,
        start: u64,
        end: u64,
    ) -> Result<tonic::Response<GetBlockHeadersResponse>, tonic::Status> {
        let request = tonic::Request::new(GetBlockHeadersRequest { start, end });
        self.rt.block_on(self.client.get_block_headers(request))
    }

    pub fn get_chain_meta(
        &mut self,
        request: impl tonic::IntoRequest<Request>,
    ) -> Result<tonic::Response<ChainMeta>, tonic::Status> {
        self.rt.block_on(self.client.get_chain_meta(request))
    }

    pub fn get_transaction(
        &mut self,
        tx_hash: &str,
    ) -> Result<tonic::Response<GetTransactionResponse>, tonic::Status> {
        let request = tonic::Request::new(TransactionHashMsg {
            tx_hash: tx_hash.into(),
        });
        self.rt.block_on(self.client.get_transaction(request))
    }

    pub fn get_receipt(
        &mut self,
        tx_hash: &str,
    ) -> Result<tonic::Response<Receipt>, tonic::Status> {
        let request = tonic::Request::new(TransactionHashMsg {
            tx_hash: tx_hash.into(),
        });
        self.rt.block_on(self.client.get_receipt(request))
    }

    pub fn send_transaction(
        &mut self,
        request: impl tonic::IntoRequest<BxhTransaction>,
    ) -> Result<tonic::Response<TransactionHashMsg>, tonic::Status> {
        self.rt.block_on(self.client.send_transaction(request))
    }

    pub fn subscribe(
        &mut self, 
        r#type: subscription_request::Type,
        extra: &[u8],
    ) -> Result<tonic::Response<tonic::codec::Streaming<Response>>, tonic::Status> {
        let request = tonic::Request::new(SubscriptionRequest {
            r#type: r#type.into(),
            extra: extra.to_vec(),
        });
        self.rt.block_on(self.client.subscribe(request))
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;
    use prost::Message;
    use super::*;

    #[test]
    fn connect_test() {
        let dst = "http://172.16.30.87:60011";
        let result = BlockingClient::connect(dst);
        assert!(result.is_ok())
    }

    #[test]
    fn get_block_test() {
        let dst = "http://172.16.30.87:60011";
        let mut client = BlockingClient::connect(dst).unwrap();
        let block = client.get_block(0, "1").unwrap();
        assert!(block.get_ref().block_header.as_ref().unwrap().number == 1)
    }

    #[test]
    fn get_block_header_test() {
        let dst = "http://172.16.30.87:60011";
        let mut client = BlockingClient::connect(dst).unwrap();
        let headers = client.get_block_headers(1, 1).unwrap();
        assert_eq!(headers
                       .get_ref()
                       .block_headers
                       .get(0)
                       .as_ref()
                       .unwrap()
                       .number, 1)
    }

    #[test]
    fn get_chain_meta_test() {
        let dst = "http://172.16.30.87:60011";
        let mut client = BlockingClient::connect(dst).unwrap();
        let req = Request { r#type: 0 };
        let res = client.get_chain_meta(req).unwrap();
        assert!(res.get_ref().height > 0)
    }

    #[test]
    fn get_transaction_test() {
        let dst = "http://172.16.30.87:60011";
        let mut client = BlockingClient::connect(dst).unwrap();
        let res = client
            .get_transaction("0x3FeD5F5C62934885c8a70072C44EB59b2425747cf6BDE395385588BddeB9C61b")
            .unwrap();
        //TODO:need checkSum
        assert_eq!(
            hex::encode(res.get_ref().tx.as_ref().unwrap().transaction_hash.to_vec()),
            "3fed5f5c62934885c8a70072c44eb59b2425747cf6bde395385588bddeb9c61b"
        )
    }

    #[test]
    fn get_receipt_test() {
        let dst = "http://172.16.30.87:60011";
        let mut client = BlockingClient::connect(dst).unwrap();
        let res = client
            .get_receipt("0x3FeD5F5C62934885c8a70072C44EB59b2425747cf6BDE395385588BddeB9C61b")
            .unwrap();
        assert_eq!(res.get_ref().status, 0)
    }

    #[test]
    fn send_transaction_test() {
        //TODO:Sdk need signature
        let dst = "http://172.16.30.87:60011";
        let mut client = BlockingClient::connect(dst).unwrap();
        let data = TransactionData {
            r#type: 0,
            amount: String::from("1000000000"),
            vm_type: 0,
            payload: vec![],
            extra: vec![],
        };
        let tx = BxhTransaction {
            version: vec![],
            from: "0xc7F999b83Af6DF9e67d0a37Ee7e900bF38b3D013"
                .as_bytes()
                .to_vec(),
            to: "0x79a1215469FaB6f9c63c1816b45183AD3624bE34"
                .as_bytes()
                .to_vec(),
            timestamp: Local::now().timestamp_nanos(),
            transaction_hash: vec![],
            payload: data.encode_to_vec(),
            nonce: 1,
            amount: String::from(""),
            typ: 0,
            signature: vec![],
            extra: vec![],
            ibtp: None,
        };
        let hash = client.send_transaction(tx).unwrap();
        assert!(!hash.get_ref().tx_hash.is_empty())
    }
}
