
# Timex: Schedule Date Time Generator

This Rust library allows you to generate date times based on a start and end date, along with other parameters. It utilizes the chrono crate for date and time manipulations.

## Demo

<https://github.com/jawahar273/timex/assets/7668497/841621b1-6325-4804-8dc3-cb5124c42a96>

Adding [interactive demo](https://timex-demo.netlify.app/) which is based on the golang and rust with Grpc.

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
- Based on daily, weekly, monthly(WIP: end date) and yearly(soon)..
- Web assembly support(Soon)

## Limitation

- Not good with nano second as this project rely on ([`Chrono`](https://docs.rs/chrono/latest/chrono/index.html#limitations)) which has know limitation.
- Recommenced to use a persistance database to store the schedule date and time.

## Usage

```rust

use timex::{schedule_date_times, ScheduleDetails};
use chrono::{DateTime, Utc};

fn main() {
    
    /// Start: Mock for schedule details
    let t = r#"
    {
        "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "day",
        "endOption": "never"
    }
    "#;
    let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();
    /// END: Mock for schedule details
    
    
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
        job_details,
        previous_scheduled_date,
        start_range,
        end_range,
    );
    dgb!(&result);
    // &result = [
    //     2024-01-05T08:28:46Z,
    //     2024-01-06T08:28:46Z,
    // ]
}


```

## License

This project is licensed under the MIT License.
