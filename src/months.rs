use crate::model::{ScheduleDetails, self};
use chrono::{DateTime, Days, offset, Utc, TimeZone, Datelike, Timelike, IsoWeek, Duration};
use anyhow::Result;

pub fn for_month(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    // end_date: DateTime<Utc>,
    range_of_days: u64,
) -> Result<Vec<DateTime<Utc>>> {
    todo!(); 
    let repeat_times = detail.repeat_every_number;

    let end_date: DateTime<Utc> = match detail.end_option {
        model::EndOption::After => {
            // TODO: confirm this logic works or not
            // let t = offset::Utc::now().checked_add_days(IsoWeek::new(detail.occurrence_value.unwrap().try_into()?)).unwrap();
            let t = offset::Utc::now() + Duration::weeks(detail.occurrence_value.unwrap().try_into()?);
            t
        },
        model::EndOption::OnThe => {
            let t = DateTime::parse_from_rfc3339(
                // detail.end_date.as_ref().unwrap().as_str()
                detail.end_date.as_ref().unwrap().as_str(),
            )?.with_timezone(&Utc);
            t
        },
        model::EndOption::Never => {
            let t = offset::Utc::now().checked_add_days(Days::new(range_of_days)).unwrap();
            t
        },
    };
    
    let mut schedule_start = scheduled_start_date_time + Duration::weeks(repeat_times as i64);
    let diff = end_date - Utc::now();
    let num_of_diff = diff.num_weeks();
    
    let mut temp: Vec< DateTime<Utc> >= vec![];
    
        for _ in 0..num_of_diff {

            temp.push(
                Utc.with_ymd_and_hms(
                    schedule_start.year(),
                    schedule_start.month(),
                    schedule_start.day(),
                    scheduled_start_date_time.hour(),
                    scheduled_start_date_time.minute(),
                    scheduled_start_date_time.second()).unwrap()
                    .with_timezone(&Utc)
            );
    
            schedule_start = schedule_start + Duration::weeks(repeat_times as i64);

        }
    
        temp.push(
            Utc.with_ymd_and_hms(
                schedule_start.year(),
                schedule_start.month(),
                schedule_start.day(),
                scheduled_start_date_time.hour(),
                scheduled_start_date_time.minute(),
                scheduled_start_date_time.second()).unwrap()
                .with_timezone(&Utc)
        );
        Ok(temp)
}