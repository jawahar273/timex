use core::fmt;

use chrono::Weekday;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RepeatEvery {
    Day,
    Week,
    Month,
    Year,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MonthOptions {
    #[serde(rename = "onThe")]
    OnThe,

    #[serde(rename = "onDay")]
    OnDay,
}

impl MonthOptions {
    pub fn from(value: String) -> Self {
        match value.as_str() {
            "onDay" => Self::OnDay,
            "onThe" => Self::OnThe,
            _ => {
                panic!("unexpected option")
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::OnDay => "onDay".to_string(),
            Self::OnThe => "onThe".to_string(),
        }
    }
}

// #[wasm_bindgen]
impl RepeatEvery {
    pub fn from(value: String) -> Self {
        match value.as_str() {
            "day" => RepeatEvery::Day,
            "month" => RepeatEvery::Month,
            "week" => RepeatEvery::Week,
            "year" => RepeatEvery::Year,
            _ => {
                panic!("unexpected option")
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RepeatEvery::Day => "day".to_string(),
            RepeatEvery::Month => "month".to_string(),
            RepeatEvery::Week => "week".to_string(),
            RepeatEvery::Year => "year".to_string(),
        }
    }
}

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

impl EndOption {
    pub fn from(value: String) -> Self {
        match value.as_str() {
            "after" => Self::After,
            "never" => Self::Never,
            "onThe" => Self::OnThe,
            _ => {
                panic!("unexpected option")
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EndOption::After => "after".to_string(),
            EndOption::Never => "never".to_string(),
            EndOption::OnThe => "onThe".to_string(),
        }
    }
}

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

impl DayCategoryFor {
    pub fn from(value: String) -> Self {
        match value.as_str() {
            "first" => Self::First,
            "second" => Self::Second,
            "third" => Self::Third,
            "fourth" => Self::Fourth,
            "last" => Self::Last,
            _ => {
                panic!("unexpected option")
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::First => "first".to_string(),
            Self::Second => "second".to_string(),
            Self::Third => "third".to_string(),
            Self::Fourth => "fourth".to_string(),
            Self::Last => "last".to_string(),
            _ => {
                panic!("unexpected option")
            }
        }
    }

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
    pub fn from(value: String) -> Self {
        match value.as_str() {
            "monday" => Self::Monday,
            "tuesday" => Self::Tuesday,
            "wednesday" => Self::Wednesday,
            "thursday" => Self::Thursday,
            "friday" => Self::Friday,
            "saturday" => Self::Saturday,
            "sunday" => Self::Sunday,
            _ => {
                panic!("unexpected option")
            }
        }
    }

    pub fn to_chrono(&self) -> Weekday {
        match self {
            Self::Monday => Weekday::Mon,
            Self::Tuesday => Weekday::Tue,
            Self::Wednesday => Weekday::Wed,
            Self::Thursday => Weekday::Thu,
            Self::Friday => Weekday::Fri,
            Self::Saturday => Weekday::Sat,
            Self::Sunday => Weekday::Sun,
        }
    }
}

/// Schedule details contain necessary field to process
/// and contain to generate scheduled date and time
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleDetails {
    /// Schedule started(initial) date should current day or greater
    #[serde(rename = "scheduledStartDateTime")]
    pub scheduled_start_date_time: String,
    
    /// Repeat calendar bases such `day`, `week`, `month` and `year`
    #[serde(rename = "repeatEvery")]
    pub repeat_every: RepeatEvery,
    
    /// Number of time the repletion should happen
    /// given value should be greater than or equal to 1
    #[serde(rename = "repeatEveryNumber")]
    pub repeat_every_number: u64,
    
    /// The type of repeat on how to recurrent schedule should stop or continue.
    /// - Should the recurrent stop [`EndOption::After`] on given recurrent count time.
    /// - Should the recurrent never stop [`EndOption::Never`]
    /// - Should the recurrent stop at the given date [`EndOption::OnThe`]
    #[serde(rename = "endOption")]
    pub end_option: EndOption,

    #[serde(rename = "endDate")]
    pub end_date: Option<String>,

    #[serde(rename = "occurrenceValue")]
    pub occurrence_value: Option<u64>,

    #[serde(rename = "weekDaysForRepeatEvery")]
    pub week_days_for_repeat_every: Option<Vec<String>>,

    #[serde(rename = "monthOptions")]
    pub month_options: Option<MonthOptions>,

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

impl fmt::Display for ScheduleDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut every: String;

        if self.repeat_every == RepeatEvery::Week {
            let week_days_for_repeat_every = self.week_days_for_repeat_every.clone().unwrap();
            every = format!(
                "{} {}",
                self.repeat_every_number,
                self.repeat_every.to_string()
            );
            if week_days_for_repeat_every.len() >= 2 {
                every = format!(
                    "{} and {}",
                    week_days_for_repeat_every[0..week_days_for_repeat_every.len() - 2].join(", "),
                    week_days_for_repeat_every[week_days_for_repeat_every.len() - 1]
                );
            } else if week_days_for_repeat_every.len() == 1 {
                every = format!("{}", week_days_for_repeat_every[0]);
            }
        } else if self.repeat_every == RepeatEvery::Month {
            every = format!(
                "{} {}",
                self.repeat_every_number,
                self.repeat_every.to_string()
            );
            if self.month_options.is_some() && self.month_options.unwrap() == MonthOptions::OnThe {
                every = format!(
                    "{} on {} {}",
                    every,
                    self.day_category_for_month.unwrap().to_string(),
                    self.week_day_for_month.unwrap().to_string(),
                );
            }
        } else if self.repeat_every == RepeatEvery::Year {
            todo!("yet to implement year logic");
        } else if self.repeat_every_number >= 2 {
            every = format!(
                "{} {}",
                self.repeat_every_number,
                self.repeat_every.to_string()
            );
        } else {
            every = format!("{}", self.repeat_every.to_string());
        }

        let start: String = format!("starting {}", self.scheduled_start_date_time);

        let mut end: String = "".to_string();
        if self.end_date.is_some() {
            end = format!("until {}", &self.end_date.clone().unwrap());
        }

        write!(f, "occurs every {every} {start} {end}")
    }
}
