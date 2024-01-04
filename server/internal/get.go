package internal

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/rs/zerolog/log"

	"github.com/gin-gonic/gin"
	pbV1 "github.com/jawahar273/timex/proto/api/v1"
)

func Get(ctx *gin.Context, c pbV1.MachineClient) {

	var form Dto
	// fmt.Println("before ", ctx.Params("details"))
	err := ctx.ShouldBindJSON(&form)
	if err != nil {
		log.Error().Msg(err.Error())
		ctx.JSON(
			http.StatusBadRequest,
			err,
		)
		return
	}

	var endDate string = ""
	if form.Details.EndDate != nil {
		endDate = form.Details.EndDate.Format(time.RFC3339)
	}

	fmt.Println("dates-->", form.PreviousScheduleDate.Format(time.RFC3339), endDate)
	f, err := c.Send(context.TODO(), &pbV1.DetailRequest{
		Details: &pbV1.ScheduleDetails{
			ScheduledStartDateTime:  form.Details.ScheduledStartDateTime.Format(time.RFC3339),
			RepeatEveryNumber:       form.Details.RepeatEveryNumber,
			RepeatEvery:             form.Details.RepeatEvery,
			EndOption:               form.Details.EndOption,
			OccurrenceValue:         form.Details.OccurrenceValue,
			EndDate:                 &endDate,
			WeekDaysForRepeatEvery:  form.Details.WeekDaysForRepeatEvery,
			MonthOptions:            form.Details.MonthOptions,
			OnDayValueForMonth:      form.Details.OnDayValueForMonth,
			DayCategoryForMonth:     form.Details.DayCategoryForMonth,
			WeekDayForMonth:         form.Details.WeekDayForMonth,
			YearOptions:             form.Details.YearOptions,
			MonthWithDayForYear:     form.Details.MonthWithDayForYear,
			OnDayValueForYear:       form.Details.OnDayValueForYear,
			DayCategoryForYear:      form.Details.DayCategoryForYear,
			WeekDayForYear:          form.Details.WeekDayForYear,
			MonthWithWeekDayForYear: form.Details.MonthWithWeekDayForYear,
		},
		PreviousScheduledDetail: form.PreviousScheduleDate.Format(time.RFC3339),
		RangedStartDate:         form.StartDate.Format(time.RFC3339),
		RangedEndDate:           form.EndDate.Format(time.RFC3339),
	})
	if err != nil {
		log.Error().Msgf("grpc machine send(): %v", err.Error())
		ctx.JSON(
			http.StatusBadRequest,
			err,
		)
		return
	}

	fmt.Println(
		"--------->",
		f,
	)
	ctx.JSON(http.StatusOK, f)
	return

}
