use crate::date_diff;

use crate::
    model::{RepeatEvery, ScheduleDetails}
;
use chrono::{
    DateTime, Utc, Duration,
};



pub fn num_diff_i64(
    detail: &ScheduleDetails,
    schedule_start: &DateTime<Utc>,
    end_date: &DateTime<Utc>,
) -> i64 {
    let repeat_times = detail.repeat_every_number;
    match detail.repeat_every {
        RepeatEvery::Day => {
            let diff = *end_date - *schedule_start;
            let num_days = diff.num_days() / (repeat_times as i64);
            num_days
        }
        RepeatEvery::Week => {
            let diff = *end_date - Utc::now();
            let num_of_diff = diff.num_weeks();
            num_of_diff
        }
        RepeatEvery::Month => {
            let diff = date_diff(&Utc::now(), &end_date);
            // day are ignore
            let y = diff.years * 12;
            if (y >= 1) {
                y + diff.months
            } else {
                diff.months
            }
        }
        RepeatEvery::Year => todo!(),
    }
}
