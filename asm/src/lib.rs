use chrono::{DateTime, Utc};
use timex::{
    model::ScheduleDetails,
    schedule_date_times,
};
use wasm_bindgen::{prelude::wasm_bindgen, *};


#[wasm_bindgen]
pub fn show_detail_in_display(_detail: JsValue) -> std::result::Result<JsValue, JsValue> {
    let detail: ScheduleDetails = match serde_wasm_bindgen::from_value(_detail) {
        Err(e) => return Err(e.to_string().into()),
        std::result::Result::Ok(v) => v,
    };

    return std::result::Result::Ok(format!("{}", detail).into());
}

#[wasm_bindgen]
pub fn find_schedule_date_time(
    _detail: JsValue,
    previous_scheduled_date: String,
    start_range_date: String,
    end_range_date: String,
) -> std::result::Result<Vec<JsValue>, JsValue> {
    let mut _previous_scheduled_date: DateTime<Utc>;
    let detail: ScheduleDetails = serde_wasm_bindgen::from_value(_detail)?;

    match DateTime::parse_from_rfc3339(&previous_scheduled_date) {
        std::result::Result::Ok(v) => _previous_scheduled_date = v.with_timezone(&Utc),
        Err(e) => return Err(e.to_string().into()),
    };

    let mut _start_range_date: DateTime<Utc>;

    match DateTime::parse_from_rfc3339(&start_range_date) {
        std::result::Result::Ok(v) => _start_range_date = v.with_timezone(&Utc),
        Err(e) => return Err(e.to_string().into()),
    };

    let mut _end_range_date: DateTime<Utc>;

    match DateTime::parse_from_rfc3339(&end_range_date) {
        std::result::Result::Ok(v) => _end_range_date = v.with_timezone(&Utc),
        Err(e) => return Err(e.to_string().into()),
    };

    let t = schedule_date_times(
        &detail,
        _previous_scheduled_date,
        _start_range_date,
        _end_range_date,
    );
    match t {
        std::result::Result::Ok(v) => std::result::Result::Ok(
            v.into_iter()
                .map(|x| x.to_rfc3339().into())
                .collect::<Vec<_>>(),
        ),
        Err(e) => Err(e.to_string().into()),
    }
}
