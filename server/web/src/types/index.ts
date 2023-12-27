	export enum RepeatEvery {
	  Day = "day",
	  Week = "week",
	  Month = "month",
	  Year = "year",
	}

	export enum EndOption {
	  After = "after",
	  Never = "never",
	  OnThe = "onThe",
	}

	export enum WeekDayForMonth {
	  Monday = "monday",
	  Tuesday = "tuesday",
	  Wednesday = "wednesday",
	  Thursday = "thursday",
	  Friday = "friday",
	  Saturday = "saturday",
	  Sunday = "sunday",
	}

	export enum DayCategoryFor {
	  First = "first",
	  Second = "second",
	  Third = "third",
	  Fourth = "fourth",
	  Last = "last",
	}


	export type ScheduleDetails = {
		scheduledStartDateTime: string
		repeatEveryNumber: number
		repeatEvery: RepeatEvery
		endOption: EndOption
		occurrenceValue?: number
		endDate?: string
		weekDaysForRepeatEvery?: string[]
		monthOptions?: string
		onDayValueForMonth?: number
		dayCategoryForMonth?: DayCategoryFor
		weekDayForMonth?: WeekDayForMonth
		yearOptions?: string
		monthWithDayForYear?: string
		onDayValueForYear?: number
		dayCategoryForYear?: string
		weekDayForYear?: string
		monthWithWeekDayForYear?: string
	}
