use core::fmt;

use chrono::Weekday;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RepeatEvery {
    Day,
    Week,
    Month,
    Year,
}

// #[wasm_bindgen]
// impl RepeatEvery {
//     pub fn to_string(&self) -> String {
//         match self {
//             RepeatEvery::Day => "day".to_string(),
//             RepeatEvery::Month => "month".to_string(),
//             RepeatEvery::Week => "week".to_string(),
//             RepeatEvery::Year => "year".to_string(),
//         }
//     }
// }

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum EndOption {
    #[serde(rename = "after")]
    After,

    #[serde(rename = "never")]
    Never,

    #[serde(rename = "onThe")]
    OnThe,
}

// #[wasm_bindgen]
// impl EndOption {
//     pub fn to_string(&self) -> String {
//         match self {
//             EndOption::After => "after".to_string(),
//             EndOption::Never => "never".to_string(),
//             EndOption::OnThe => "onThe".to_string(),
//         }
//     }
// }

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
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

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
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
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleDetails {
    #[serde(rename = "scheduledStartDateTime")]
    pub scheduled_start_date_time: String,
    
    #[serde(rename = "repeatEveryNumber")]
    pub repeat_every_number: u64,
    
    #[serde(rename = "repeatEvery")]
    pub repeat_every: RepeatEvery,

    #[serde(rename = "endOption")]
    pub end_option: EndOption,
    
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    
    #[serde(rename = "occurrenceValue")]
    pub occurrence_value: Option<u64>,
    
    #[serde(rename="weekDaysForRepeatEvery")]
    pub week_days_for_repeat_every: Option<Vec<String>>,
    
    #[serde(rename = "monthOptions")]
    pub month_options: Option<String>,

    #[serde(rename = "onDayValueForMonth")]
    pub on_day_value_for_month: Option<i64>,

    #[serde(rename = "dayCategoryForMonth")]
    pub day_category_for_month: Option<DayCategoryFor>,
    
    #[serde(rename = "weekDayForMonth")]
    pub week_day_for_month: Option<WeekDayForMonth>,
    
    #[serde(rename = "yearOptions")]
    pub year_options: Option<String>,

    #[serde(rename = "monthWithDayForYear")]
    pub month_with_day_for_year: Option<String>,
    
    #[serde(rename = "onDayValueForYear")]
    pub on_day_value_for_year: Option<i64>,
    
    #[serde(rename = "dayCategoryForYear")]
    pub day_category_for_year: Option<String>,
    
    #[serde(rename = "weekDayForYear")]
    pub week_day_for_year: Option<String>,

    #[serde(rename = "monthWithWeekDayForYear")]
    pub month_with_week_day_for_year: Option<String>,
}
