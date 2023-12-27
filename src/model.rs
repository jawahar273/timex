use core::fmt;
use std::fmt::write;

use chrono::Weekday;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RepeatEvery {
    Day,
    Week,
    Month,
    Year,
}

impl RepeatEvery {
    pub fn as_str<'a>(s: &'a RepeatEvery) -> &'a str {
        match s {
            RepeatEvery::Day => "day",
            RepeatEvery::Month => "month",
            RepeatEvery::Week => "week",
            RepeatEvery::Year => "year",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EndOption {
    #[serde(rename = "after")]
    After,

    #[serde(rename = "never")]
    Never,

    #[serde(rename = "onThe")]
    OnThe,
}

impl EndOption {
    pub fn as_str<'a>(s: &'a EndOption) -> &'a str {
        match s {
            EndOption::After => "after",
            EndOption::Never => "never",
            EndOption::OnThe => "onThe",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DayCategoryFor {
    #[serde(rename = "first")]
    First,

    #[serde(rename = "second")]
    Second,

    #[serde(rename = "third")]
    Third,
    
    #[serde(rename = "fourth")]
    Fourth,
    
    #[serde(rename = "last")]
    Last, 
}

impl  DayCategoryFor {
    pub fn to_week_in_month(&self) -> i32 {
        match self {
            Self::First => 0,
            Self::Second => 1,
            Self::Third => 2,
            Self::Fourth => 3,
            Self::Last => -1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WeekDayForMonth {
    #[serde(rename = "monday")]
    Monday,
    
    #[serde(rename = "tuesday")]
    Tuesday,
    
    #[serde(rename = "wednesday")]
    Wednesday, 
    
    #[serde(rename = "thursday")]
    Thursday, 
    
    #[serde(rename = "friday")]
    Friday,   
    
    #[serde(rename = "saturday")]
    Saturday,
    
    #[serde(rename = "sunday")]
    Sunday,
}

impl fmt::Display for WeekDayForMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl WeekDayForMonth {
    pub fn to_chrono(&self) -> Weekday {
        match self {
           Self::Monday =>  Weekday::Mon,
           Self::Tuesday => Weekday::Tue,
           Self::Wednesday => Weekday::Wed,
           Self::Thursday => Weekday::Thu,
           Self::Friday => Weekday::Fri,
           Self::Saturday => Weekday::Sat,
           Self::Sunday => Weekday::Sun,
        }
    }
}

// Default
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleDetails {
    pub scheduled_start_date_time: String,
    pub repeat_every_number: u64,
    pub repeat_every: RepeatEvery,
    pub end_option: EndOption,
    pub end_date: Option<String>,
    pub occurrence_value: Option<u64>,
    #[serde(default)]
    pub week_days_for_repeat_every: Option<Vec<String>>,
    pub month_options: Option<String>,
    pub on_day_value_for_month: Option<i64>,
    pub day_category_for_month: Option<DayCategoryFor>,
    pub week_day_for_month: Option<WeekDayForMonth>,
    pub year_options: Option<String>,
    pub month_with_day_for_year: Option<String>,
    pub on_day_value_for_year: Option<i64>,
    pub day_category_for_year: Option<String>,
    pub week_day_for_year: Option<String>,
    pub month_with_week_day_for_year: Option<String>,
}
