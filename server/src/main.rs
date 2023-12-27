use chrono::{Utc, DateTime};
use serde::Serialize;
use serde_json::{self, Value, json};
use timex_core::schedule_date_times;
use timex_core::model::ScheduleDetails;
use tonic_health::server::HealthReporter;
use std::{fmt, net::SocketAddr, time::Duration};

use tonic::{transport::Server, Request, Response, Status};

use std::default::Default;

use proto::{
    DetailRequest, DetailResponse,
    machine_server::{
        Machine as Timex,
        MachineServer as TimexServer
    }
};

pub mod proto {
    tonic::include_proto!("timex");
}

type TimexResult<T> = Result<Response<T>, Status>;


async fn core_call() {
    let sc = r#"
    {
       "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
       "repeatEveryNumber": 1,
       "repeatEvery": "day",
       "endOption": "never"
     }
    "#;
     let job_details: ScheduleDetails = serde_json::from_str(sc).unwrap();
     let temp = schedule_date_times(
         &job_details,
         DateTime::parse_from_rfc3339("2023-12-26T15:01:21.214570Z").unwrap().with_timezone(&Utc),
         DateTime::parse_from_rfc3339("2023-12-25T00:00:00Z").unwrap().with_timezone(&Utc),
         DateTime::parse_from_rfc3339("2023-12-31T00:00:00Z").unwrap().with_timezone(&Utc),
     );
 
     dbg!(temp);
}

#[derive(Serialize, Debug, Clone)]
enum StatusKey {
    Ok
}

impl fmt::Display for StatusKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Serialize, Clone)]
struct HealthResponse {
    status: StatusKey
}


type TsserverService = TimexServer<Service>;

async fn twiddle_service_status(mut reporter: HealthReporter) {
    let mut iter = 0u64;
    loop {
        iter += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;

        if iter % 2 == 0 {
            reporter.set_serving::<TsserverService>().await;
        } else {
            reporter.set_not_serving::<TsserverService>().await;
        };
    }
}

    
#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {
    // core_call();
    
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<TsserverService>()
        .await;
    
    tokio::spawn(twiddle_service_status(health_reporter.clone()));
    
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    dbg!(&addr);
    let svc = Service::default();
    let t = TimexServer::new(svc);
    Server::builder()
        .add_service(t)
        .serve_with_shutdown(addr, async {
            tokio::signal::ctrl_c().await.unwrap();
        })
        // .serve_with_incoming_shutdown(address, async {
        //     tokio::signal::ctrl_c().await.unwrap();
        // })
        .await?;
    
    Ok(())
}

async fn shutdown() {
    tokio::signal::ctrl_c().await.expect("Graceful shutdown")
}


// impl proto::ScheduleDetails {
//     to_sered
// }

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl Timex for Service {
    async fn send(&self, request: Request<DetailRequest>) -> TimexResult<DetailResponse> {

        let y= request.into_inner();
        
       let details = y.details.expect("schedule details is required");
        
        let sch = schedule_date_times(
            &details,
            y.previous_scheduled_detail,
            y.ranged_start_date,
            y.ranged_end_date,
         );
 
        Ok(
            Response::new(
                DetailResponse{
                    scheduled_date_time: sch,
                }   
            )
        )
    }
    
    async fn send_test(&self, request: Request<DetailResponse>) -> TimexResult<DetailResponse> {
        let mut t: Vec<String> = Vec::new();
        t.push("jklfdsjsk".to_string());
        
        Ok(
            Response::new(
                DetailResponse{
                    scheduled_date_time: t
                }   
            )
        )
    }
}