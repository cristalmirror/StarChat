use tonic::{Request, Response, Status};

/*
   these module is generated atomatly for tonic-build in build_proto.rs
   using like base .proto
*/
pub mod replication {

    tonic::include_proto!("replication");
}

use replication::{
    server_link_server::{ServerLink, ServerLinkServer},
    ReplicateRequest, ReplicateAck,
};

#[derive(Debug, Default)]
pub struct MyServerLink {}

#[tonic::async_trait]
impl ServerLink for MyServerLink {
    async fn replicate_message(
        &self,
        request: Request<ReplicateRequest>,)
        -> Result<Response<ReplicateAck>, Status> {

        /*request refactor in a ReplicationRequest*/
        let req = request.into_inner();
        println!("[Reply Resept]: conbersation_id={}, message_id={}", req.conversation_id, req.message_id);

        let ack = ReplicateAck {
            success: true,
            detail: "replicate".to_string(),
        };
        Ok(Response::new(ack))
    }
}
