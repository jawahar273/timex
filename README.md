# Timex

Generate range of schedule date and time based on the
parameter provided.

## Feature

- frequency
- Time range
- Occurrence
- Based on daily, weekly, monthly(WIP: end date) and yearly(soon)..

## Limitation

- Not good with nano second as this project rely on ([`Chrono`](https://docs.rs/chrono/latest/chrono/index.html#limitations)) which has know limitation.
- Recommenced to use a persistance database to store the schedule date and time.
