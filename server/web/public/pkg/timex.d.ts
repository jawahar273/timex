/* tslint:disable */
/* eslint-disable */
/**
* @param {any} _detail
* @returns {any}
*/
export function showDetailInDisplay(_detail: any): any;
/**
* @param {any} _detail
* @param {string} previous_scheduled_date
* @param {string} start_range_date
* @param {string} end_range_date
* @returns {any[]}
*/
export function find_schedule_date_time(_detail: any, previous_scheduled_date: string, start_range_date: string, end_range_date: string): any[];
/**
*/
export enum RepeatEvery {
  Day = 0,
  Week = 1,
  Month = 2,
  Year = 3,
}
/**
*/
export enum EndOption {
  After = 0,
  Never = 1,
  OnThe = 2,
}
/**
*/
export enum DayCategoryFor {
  First = 0,
  Second = 1,
  Third = 2,
  Fourth = 3,
  Last = 4,
}
/**
* List of weekday in a week
*/
export enum WeekDayForMonth {
  Monday = 0,
  Tuesday = 1,
  Wednesday = 2,
  Thursday = 3,
  Friday = 4,
  Saturday = 5,
  Sunday = 6,
}
/**
*/
export enum MonthOptions {
  OnThe = 0,
  OnDay = 1,
}
/**
* Schedule details contain necessary field to process
* and contain to generate scheduled date and time
*/
export class ScheduleDetails {
  free(): void;
/**
* Order on which week to mark as schedule date-time
* - First week, ... Last week
*/
  day_category_for_month?: DayCategoryFor;
/**
*/
  day_category_for_year?: string;
/**
* End date to stop the schedule date generated
*/
  end_date?: string;
/**
* The type of repeat on how to recurrent schedule should stop or continue.
* - Should the recurrent stop [`EndOption::After`] on given recurrent count time.
* - Should the recurrent never stop [`EndOption::Never`]
* - Should the recurrent stop at the given date [`EndOption::OnThe`]
*/
  end_option: EndOption;
/**
* WIP
*/
  month_options?: MonthOptions;
/**
*/
  month_with_day_for_year?: string;
/**
*/
  month_with_week_day_for_year?: string;
/**
* Number of time the schedule generate
*/
  occurrence_value?: bigint;
/**
* schedule the date for the given day.
* 
* - For example if value set to `5` the date will generated
*     as `2023-11-05T00:00:00.00Z`.
* - If the value is get to 31 the end date for the month will be always
* has correct end date month. For example January => 31, February => 28/29(accordingly to leap year), March => 31, April => 30
* 
* # Panics
* 
* If the value is greater than or equal 32 [`crate::errors::ScheduleError::DaysWithMoreThan31AreNotAllowed`]
* panic will be thrown
*/
  on_day_value_for_month?: bigint;
/**
*/
  on_day_value_for_year?: bigint;
/**
* Repeat calendar bases such `day`, `week`, `month` and `year`
*/
  repeat_every: RepeatEvery;
/**
* Number of time the repletion should happen
* given value should be greater than or equal to 1
*/
  repeat_every_number: bigint;
/**
* Schedule started(initial) date should current day or greater
*/
  scheduled_start_date_time: string;
/**
* Similar to [`ScheduleDetails::week_days_for_repeat_every`]
*/
  week_day_for_month?: WeekDayForMonth;
/**
*/
  week_day_for_year?: string;
/**
* List of week days [`WeekDayForMonth`]
*/
  week_days_for_repeat_every?: any[];
/**
*/
  year_options?: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_scheduledetails_free: (a: number) => void;
  readonly __wbg_get_scheduledetails_scheduled_start_date_time: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_scheduled_start_date_time: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_repeat_every: (a: number) => number;
  readonly __wbg_set_scheduledetails_repeat_every: (a: number, b: number) => void;
  readonly __wbg_get_scheduledetails_repeat_every_number: (a: number) => number;
  readonly __wbg_set_scheduledetails_repeat_every_number: (a: number, b: number) => void;
  readonly __wbg_get_scheduledetails_end_option: (a: number) => number;
  readonly __wbg_set_scheduledetails_end_option: (a: number, b: number) => void;
  readonly __wbg_get_scheduledetails_end_date: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_end_date: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_occurrence_value: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_occurrence_value: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_week_days_for_repeat_every: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_week_days_for_repeat_every: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_month_options: (a: number) => number;
  readonly __wbg_set_scheduledetails_month_options: (a: number, b: number) => void;
  readonly __wbg_get_scheduledetails_on_day_value_for_month: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_on_day_value_for_month: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_day_category_for_month: (a: number) => number;
  readonly __wbg_set_scheduledetails_day_category_for_month: (a: number, b: number) => void;
  readonly __wbg_get_scheduledetails_week_day_for_month: (a: number) => number;
  readonly __wbg_set_scheduledetails_week_day_for_month: (a: number, b: number) => void;
  readonly __wbg_get_scheduledetails_year_options: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_year_options: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_month_with_day_for_year: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_month_with_day_for_year: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_on_day_value_for_year: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_on_day_value_for_year: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_day_category_for_year: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_day_category_for_year: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_week_day_for_year: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_week_day_for_year: (a: number, b: number, c: number) => void;
  readonly __wbg_get_scheduledetails_month_with_week_day_for_year: (a: number, b: number) => void;
  readonly __wbg_set_scheduledetails_month_with_week_day_for_year: (a: number, b: number, c: number) => void;
  readonly showDetailInDisplay: (a: number, b: number) => void;
  readonly find_schedule_date_time: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
