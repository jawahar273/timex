use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{self, json, Value};
use std::env;
use std::{fmt, net::SocketAddr, time::Duration};
use timex::model::{
    DayCategoryFor, EndOption, MonthOptions, ScheduleDetails as TScheduleDetails, WeekDayForMonth,
};
use timex::{model::RepeatEvery, schedule_date_times};
use tonic_health::server::HealthReporter;

use tonic::{transport::Server, Request, Response, Status};

use std::default::Default;

use proto::{
    machine_server::{Machine as Timex, MachineServer as TimexServer},
    DetailRequest, DetailResponse,
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
    let job_details: TScheduleDetails = serde_json::from_str(sc).unwrap();
    let temp = schedule_date_times(
        &job_details,
        DateTime::parse_from_rfc3339("2023-12-26T15:01:21.214570Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2023-12-25T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2023-12-31T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
    );

    dbg!(temp);
}

#[derive(Serialize, Debug, Clone)]
enum StatusKey {
    Ok,
}

impl fmt::Display for StatusKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Clone)]
struct HealthResponse {
    status: StatusKey,
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // core_call();

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<TsserverService>().await;

    tokio::spawn(twiddle_service_status(health_reporter.clone()));
    
    let host: String = env::var("R_HOST")?;
    let addr: SocketAddr = host.parse().unwrap();

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

impl proto::ScheduleDetails {
    fn to_sered(&self) -> TScheduleDetails {
        let pr = self.clone();

        let repeat_every = RepeatEvery::from(pr.repeat_every);
        let end_option = EndOption::from(pr.end_option.expect("wrong option for end options"));
        let week_days_for_repeat_every = Some(pr.week_days_for_repeat_every);
        let day_category_for_month = match pr.day_category_for_month {
            Some(v) => Some(DayCategoryFor::from(v)),
            None => None,
        };
        let week_day_for_month = match pr.week_day_for_month {
            Some(v) => {
                dbg!(&v);
                Some(WeekDayForMonth::from(v))
            }
            None => None,
        };

        let month_options: Option<MonthOptions> = match pr.month_options {
            Some(v) => Some(MonthOptions::from(v)),
            None => None,
        };

        return TScheduleDetails {
            scheduled_start_date_time: pr.scheduled_start_date_time,
            repeat_every_number: pr.repeat_every_number,
            repeat_every,
            end_option: end_option,
            end_date: pr.end_date,
            occurrence_value: pr.occurrence_value,
            week_days_for_repeat_every: week_days_for_repeat_every,
            month_options: month_options,
            on_day_value_for_month: pr.on_day_value_for_month,
            day_category_for_month: day_category_for_month,
            week_day_for_month: week_day_for_month,
            year_options: pr.year_options,
            month_with_day_for_year: pr.month_with_day_for_year,
            on_day_value_for_year: pr.on_day_value_for_year,
            day_category_for_year: pr.day_category_for_year,
            week_day_for_year: pr.week_day_for_year,
            month_with_week_day_for_year: pr.month_with_week_day_for_year,
        };
    }
}

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl Timex for Service {
    async fn send(&self, request: Request<DetailRequest>) -> TimexResult<DetailResponse> {
        let y = request.into_inner();

        let details = y.details.expect("schedule details is required");

        let previous_scheduled_detail =
            chrono::DateTime::parse_from_rfc3339(&y.previous_scheduled_detail)
                .expect("previous scheduled date error")
                .with_timezone(&Utc);

        let ranged_start_date = chrono::DateTime::parse_from_rfc3339(&y.ranged_start_date)
            .expect("start range date error")
            .with_timezone(&Utc);

        let ranged_end_date = chrono::DateTime::parse_from_rfc3339(&y.ranged_end_date)
            .expect("end range date error")
            .with_timezone(&Utc);
        dbg!(&y.previous_scheduled_detail);

        let sch = schedule_date_times(
            &details.to_sered(),
            previous_scheduled_detail,
            ranged_start_date,
            ranged_end_date,
        );

        match sch {
            Ok(v) => Ok(Response::new(DetailResponse {
                scheduled_date_time: v.iter().map(|t| t.to_rfc3339()).collect(),
            })),
            Err(e) => Err(Status::new(tonic::Code::Internal, e.to_string())),
        }
    }

    async fn send_test(&self, request: Request<DetailResponse>) -> TimexResult<DetailResponse> {
        return Err(Status::aborted("testing error"));
    }
}
