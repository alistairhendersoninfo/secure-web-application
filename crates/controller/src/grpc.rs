use anyhow::Result;
use sqlx::Pool;
use sqlx::Postgres;
use tonic::{transport::{Server, ServerTlsConfig, Identity, Certificate}, Request, Response, Status};

pub mod proto {
    tonic::include_proto!("swap.v1");
}

use proto::enrollment_service_server::{EnrollmentService, EnrollmentServiceServer};
use proto::heartbeat_service_server::{HeartbeatService, HeartbeatServiceServer};
use proto::desired_state_service_server::{DesiredStateService, DesiredStateServiceServer};
use proto::plan_service_server::{PlanService, PlanServiceServer};
use proto::log_service_server::{LogService, LogServiceServer};

use proto::{EnrollmentRequest, EnrollmentResponse, HeartbeatRequest, HeartbeatResponse, DesiredStateRequest, DesiredStateUpdate, ApplyPlanRequest, ApplyPlanResult, LogEnvelope, StreamAck};

#[derive(Clone)]
pub struct ControllerSvc {
    pub db: Pool<Postgres>,
}

#[tonic::async_trait]
impl EnrollmentService for ControllerSvc {
    async fn enroll(&self, _req: Request<EnrollmentRequest>) -> Result<Response<EnrollmentResponse>, Status> {
        let resp = EnrollmentResponse { agent_id: String::new(), certificate: vec![], ca_chain: vec![] };
        Ok(Response::new(resp))
    }
}

#[tonic::async_trait]
impl HeartbeatService for ControllerSvc {
    async fn heartbeat(&self, _req: Request<HeartbeatRequest>) -> Result<Response<HeartbeatResponse>, Status> {
        Ok(Response::new(HeartbeatResponse { next_interval_ms: 30_000 }))
    }
}

#[tonic::async_trait]
impl DesiredStateService for ControllerSvc {
    async fn pull(&self, _req: Request<DesiredStateRequest>) -> Result<Response<DesiredStateUpdate>, Status> {
        Ok(Response::new(DesiredStateUpdate { version: String::new(), configs: vec![] }))
    }
}

#[tonic::async_trait]
impl PlanService for ControllerSvc {
    async fn apply(&self, req: Request<ApplyPlanRequest>) -> Result<Response<ApplyPlanResult>, Status> {
        let plan_id = req.into_inner().plan_id;
        Ok(Response::new(ApplyPlanResult { plan_id, success: true, message: String::new(), details_json: String::new() }))
    }
}

#[tonic::async_trait]
impl LogService for ControllerSvc {
    async fn stream(&self, mut req: Request<tonic::Streaming<LogEnvelope>>) -> Result<Response<StreamAck>, Status> {
        let mut last_seq = 0u64;
        while let Some(_env) = req.get_mut().message().await.transpose()? {
            last_seq += 1;
        }
        Ok(Response::new(StreamAck { ack_seq: last_seq }))
    }
}

pub async fn serve_grpc(
    addr: std::net::SocketAddr,
    tls_identity: Identity,
    client_ca: Certificate,
    db: Pool<Postgres>,
) -> Result<()> {
    let svc = ControllerSvc { db };

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(tls_identity).client_ca_root(client_ca))?
        .add_service(EnrollmentServiceServer::new(svc.clone()))
        .add_service(HeartbeatServiceServer::new(svc.clone()))
        .add_service(DesiredStateServiceServer::new(svc.clone()))
        .add_service(PlanServiceServer::new(svc.clone()))
        .add_service(LogServiceServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}

