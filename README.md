
# Timex: Schedule Date Time Generator

This Rust library enables the generation of date-time schedules based on specified start and end dates, along with configurable intervals. It leverages the chrono crate for robust date and time manipulations.

## Demo

<https://github.com/jawahar273/timex/assets/7668497/841621b1-6325-4804-8dc3-cb5124c42a96>


## Usage

```rust


use timex::{
    schedule_date_times,
    model::ScheduleDetails
};
use chrono::{DateTime, Utc};
use serde_json;


fn main() {

    // Start: Mock for schedule details
    let t = r#"
    {
        "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "day",
        "endOption": "never"
    }
    "#;
    let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();
    // END: Mock for schedule details


    let previous_scheduled_date = DateTime::parse_from_rfc3339("2024-01-03T00:00:00Z")
                        .unwrap()
                        .with_timezone(&Utc);
    let start_range = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                        .unwrap()
                        .with_timezone(&Utc);
    let end_range = DateTime::parse_from_rfc3339("2024-01-07T00:00:00Z")
                        .unwrap()
                        .with_timezone(&Utc);
    let result = schedule_date_times(
        &job_details,
        previous_scheduled_date,
        start_range,
        end_range,
    );
    println!("{:?}",&result.unwrap());
    // [
    // 2024-01-04T00:00:00Z,
    // 2024-01-05T00:00:00Z,
    // 2024-01-06T00:00:00Z,
    // 2024-01-07T00:00:00Z
    // ]
}
```

For recreated demo check `server/README.md`.

<!--
## Installation

To install this library, add the following line to your Cargo.toml file under [dependencies] section:
```rust
[dependencies]
timex = "0.0.1"

```
-->

## Feature

- Time range
- Occurrence
- Based on daily, weekly, monthly and yearly(soon)..
- Web assembly support

## Limitation

- Not good with nano second as this project rely on ([`Chrono`](https://docs.rs/chrono/latest/chrono/index.html#limitations)) which has know limitation.
- Recommenced to use a persistance database to store the schedule date and time.


## License

This project is licensed under the MIT License.

## About this project

This project will follow the conversion of [semver](https://semver.org) versions and for the commit [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/#summary) to track change log.
