// use anyhow::{bail, Result};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ScheduleError {
    #[error("Days repeat number can not be more than 31")]
    DaysWithMoreThan31AreNotAllowed(),
    #[error("Unexpected error from Scheduler")]
    Unknown(),
}
