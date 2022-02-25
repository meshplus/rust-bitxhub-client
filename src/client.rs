use pb::chain_broker_client::ChainBrokerClient;
use tokio::runtime::{Builder, Runtime};

pub mod pb {
    tonic::include_proto!("pb");
}

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
        typ: i32, value: &str
    ) -> Result<tonic::Response<pb::Block>, tonic::Status> {
        let request = tonic::Request::new(pb::GetBlockRequest {
            r#type: typ,
            value: value.into()
        });
        self.rt.block_on(self.client.get_block(request))
    }

    pub fn get_block_headers(
        &mut self,
        start: u64, end: u64
    ) -> Result<tonic::Response<pb::GetBlockHeadersResponse>, tonic::Status> {
        let request = tonic::Request::new(pb::GetBlockHeadersRequest {
            start,
            end
        });
        self.rt.block_on(self.client.get_block_headers(request))
    }


    pub fn get_chain_meta(
        &mut self,
        request: impl tonic::IntoRequest<pb::Request>,
    ) -> Result<tonic::Response<pb::ChainMeta>, tonic::Status> {
        self.rt.block_on(self.client.get_chain_meta(request))
    }

    pub fn get_transaction(
        &mut self,
        tx_hash: &str,
    ) -> Result<tonic::Response<pb::GetTransactionResponse>, tonic::Status> {
        let request = tonic::Request::new(pb::TransactionHashMsg{
           tx_hash:tx_hash.into()
        });
        self.rt.block_on(self.client.get_transaction(request))
    } 

    pub fn get_receipt(
        &mut self,
        tx_hash: &str,
    ) -> Result<tonic::Response<pb::Receipt>, tonic::Status> {
        let request = tonic::Request::new(pb::TransactionHashMsg{
            tx_hash:tx_hash.into()
         });
        self.rt.block_on(self.client.get_receipt(request))
    }

    pub fn send_transaction(
        &mut self,
        request: impl tonic::IntoRequest<pb::BxhTransaction>,
    ) -> Result<tonic::Response<pb::TransactionHashMsg>, tonic::Status> {
        self.rt.block_on(self.client.send_transaction(request))
    }

}



