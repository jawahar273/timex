import { RepeatEvery, ScheduleDetails } from "@timex/types";

export type TInputs = ScheduleDetails & {
  repeatEveryType: RepeatEvery;
  type: Types;
  name: string;
};

export enum Types {
  EVERY_DAY = "every day non stop",
  EVERY_WEEK = "every week non stop",
  EVERY_MONTH = "every month non stop",
  EVERY_MONTH_2 = "every 2 month non stop",
}
