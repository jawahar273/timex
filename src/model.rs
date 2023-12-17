use serde::{Serialize, Deserialize};



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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

// Default
#[derive( Debug, Clone, Serialize, Deserialize)]
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
    pub day_category_for_month: Option<String>,
    pub week_day_for_month: Option<String>,
    pub year_options: Option<String>,
    pub month_with_day_for_year: Option<String>,
    pub on_day_value_for_year: Option<i64>,
    pub day_category_for_year: Option<String>,
    pub week_day_for_year: Option<String>,
    pub month_with_week_day_for_year: Option<String>,
}

