package internal

import (
	"github.com/gin-gonic/gin"
	pbV1 "github.com/jawahar273/timex/proto/api/v1"
)

func SetupRoutesV1(
	router *gin.RouterGroup,
	m pbV1.MachineClient,
) {
	schedulerRouter := router.Group("schedule")
	schedulerRouter.POST("/", func(ctx *gin.Context) {
		Get(ctx, m)
		return
	})
}
