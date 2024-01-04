package internal

import "time"

type ScheduleDetailsDto struct {
	ScheduledStartDateTime  time.Time  `form:"scheduledStartDateTime" binding:"required"`
	RepeatEveryNumber       uint64     `form:"repeatEveryNumber" binding:"required"`
	RepeatEvery             string     `form:"repeatEvery" binding:"required"`
	EndOption               *string    `form:"endOption" binding:"required"`
	OccurrenceValue         *uint64    `form:"occurrenceValue,omitempty"`
	EndDate                 *time.Time `form:"endDate,omitempty"`
	WeekDaysForRepeatEvery  []string   `form:"weekDaysForRepeatEvery,omitempty"`
	MonthOptions            *string    `form:"monthOptions,omitempty"`
	OnDayValueForMonth      *int64     `form:"onDayValueForMonth,omitempty"`
	DayCategoryForMonth     *string    `form:"dayCategoryForMonth,omitempty"`
	WeekDayForMonth         *string    `form:"weekDayForMonth,omitempty"`
	YearOptions             *string    `form:"yearOptions,omitempty"`
	MonthWithDayForYear     *string    `form:"monthWithDayForYear,omitempty"`
	OnDayValueForYear       *int64     `form:"onDayValueForYear,omitempty"`
	DayCategoryForYear      *string    `form:"dayCategoryForYear,omitempty"`
	WeekDayForYear          *string    `form:"weekDayForYear,omitempty"`
	MonthWithWeekDayForYear *string    `form:"monthWithWeekDayForYear,omitempty"`
}

type Dto struct {
	Details              *ScheduleDetailsDto `form:"details" binding:"required"`
	PreviousScheduleDate *time.Time          `form:"previousScheduleDate" binding:"required" time_utc:"1"`
	StartDate            *time.Time          `form:"startDate" binding:"required,ltefield=EndDate" time_utc:"1"`
	EndDate              *time.Time          `form:"endDate" binding:"required" time_utc:"1"`
}
