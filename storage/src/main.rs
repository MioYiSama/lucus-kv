use std::error::Error;

use tonic::{Request, Response, Status, transport::Server};

unsafe extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

pub mod proto {
    tonic::include_proto!("net.mioyi.lucuskv"); // The string specified here must match the proto package name
}

struct A;

#[tonic::async_trait]
impl proto::storage_service_server::StorageService for A {
    async fn health(
        &self,
        _request: Request<proto::HealthRequest>,
    ) -> Result<Response<proto::HealthResponse>, Status> {
        Ok(Response::new(proto::HealthResponse {
            value: unsafe { add(1, 2) } == 3,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Server::builder()
        .add_service(proto::storage_service_server::StorageServiceServer::new(A))
        .serve("0.0.0.0:50051".parse()?)
        .await?;

    Ok(())
}
