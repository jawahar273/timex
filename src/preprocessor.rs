use anyhow::{Result, Ok, bail};

use crate::{
    model::{ScheduleDetails, RepeatEvery},
    errors::ScheduleError
};



pub fn simple_preprocessing(detail: &ScheduleDetails) -> Result<()> {
    // preprocessing step
    match detail.repeat_every {
        RepeatEvery::Day => {
            if detail.repeat_every_number >= 32 {
                bail!(ScheduleError::DaysWithMoreThan31AreNotAllowed());
            }
            Ok(())
        }
        RepeatEvery::Week => {
            Ok(())
        }
        RepeatEvery::Month => {
            if detail.on_day_value_for_month.is_some()
                && detail.on_day_value_for_month.unwrap() >= 32
            {
                bail!(ScheduleError::DaysWithMoreThan31AreNotAllowed());
            }
            Ok(())
        }
        RepeatEvery::Year => {
            Ok(())
        }
    }
    
}