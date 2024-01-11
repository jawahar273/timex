package main

import (
	"context"
	"fmt"
	"net/http"
	"os/signal"
	"syscall"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-contrib/gzip"
	"github.com/gin-contrib/logger"
	swaggerfiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
	"github.com/swaggo/swag/example/basic/docs"

	"github.com/gin-contrib/requestid"
	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog/log"

	"github.com/spf13/viper"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	internal "github.com/jawahar273/timex/internal"
	pbV1 "github.com/jawahar273/timex/proto/api/v1"
)

func GrpcClient(ctx context.Context, addr string) (*grpc.ClientConn, error) {
	conn, err := grpc.DialContext(ctx, addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	return conn, err
}

const successful = "ok"
const DEVELOPMENT = "development"

func main() {

	// viper.SetDefault("GIN_MODE", constants.PRODUCTION)
	viper.AutomaticEnv()
	v := viper.New()

	v.SetDefault("GIN_MODE", DEVELOPMENT)
	v.SetDefault("PORT", 8300)
	v.SetDefault("R_HOST", "[::1]")

	if v.GetString("GIN_MODE") == DEVELOPMENT {
		log.Info().Msg("Development environment")

	} else {
		gin.SetMode(gin.ReleaseMode)
	}
	log.Info().Msgf("env: %s", v.GetString("GIN_MODE"))

	var port = v.GetInt("PORT")
	log.Info().Msgf("server run the port %v", port)

	// Create context that listens for the interrupt signal from the OS.
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	router := gin.Default()

	router.Use(logger.SetLogger())
	router.Use(gin.Recovery())

	router.Use(gzip.Gzip(gzip.DefaultCompression))
	router.Use(requestid.New())

	config := cors.Config{
		// AllowOrigins:     []string{viper.GetString("ALLOW_DOMAIN")},
		AllowMethods:     []string{"GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"},
		AllowHeaders:     []string{"Origin", "Content-Length", "Content-Type"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
		AllowAllOrigins:  true,
		// AllowOriginFunc: func(origin string) bool {
		// 	return false
		// },
		MaxAge: 12 * time.Hour,
	}

	if v.GetString("GIN_MODE") == DEVELOPMENT {
		docs.SwaggerInfo.BasePath = "/api/"

		router.GET("/swagger/*any", ginSwagger.WrapHandler(swaggerfiles.Handler))

	}

	router.Use(cors.New(config))

	// Default content type
	router.Use(func(ctx *gin.Context) {
		ctx.Header("Content-Type", "application/json")
	})

	apiV1 := router.Group("/api/v1")

	router.GET("/api/healthz", func(c *gin.Context) {

		var status = successful

		// err := driver.VerifyConnectivity(ctx)
		// if err != nil {
		// 	status = failed
		// 	log.Fatal().Err(err).Msg("inside health: unable to connect with neo4j")
		// }
		// err = mongoDriver.Disconnect(ctx)
		// if err != nil {
		// 	status = failed
		// 	log.Fatal().Err(err).Msg("inside health: unable to connect with mongodb")
		// }

		c.JSON(http.StatusOK, gin.H{
			"message": status,
		})
	})

	router.NoRoute(func(ctx *gin.Context) {
		ctx.AbortWithStatusJSON(
			http.StatusNotFound,
			gin.H{
				"message": "path not found",
			},
		)
	})

	var err error
	var grpcConn *grpc.ClientConn = nil

	srv := &http.Server{
		Addr:    fmt.Sprintf(":%d", port),
		Handler: router,
	}
	// Initializing the server in a goroutine so that
	// it won't block the graceful shutdown handling below
	// Reference:
	// [https://github.com/gin-gonic/examples/blob/master/graceful-shutdown/graceful-shutdown/notify-with-context/server.go]
	go func() {
		log.Info().Msg("start the server")

		grpcConn, err = GrpcClient(ctx, v.GetString("R_HOST")+":50051")
		// pbV1.send

		if err != nil {
			log.Fatal().Msgf("grpc listen: %s\n", err)
		}

		c := pbV1.NewMachineClient(grpcConn)
		internal.SetupRoutesV1(
			apiV1,
			c,
		)

		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatal().Msgf("listen: %s\n", err)
		}

	}()

	// Listen for the interrupt signal.
	<-ctx.Done()

	// Restore default behavior on the interrupt signal and notify user of shutdown.
	stop()

	log.Info().Msg("shutting down gracefully, press Ctrl+C again to force")

	// The context is used to inform the server it has 5 seconds to finish
	// the request it is currently handling
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	log.Info().Msg("starting to close the server")
	if err := srv.Shutdown(ctx); err != nil {
		log.Fatal().Err(err).Msg("Server forced to shutdown")
	}

	log.Info().Msg("starting to close the grpc server")
	if err := grpcConn.Close(); err != nil {
		log.Fatal().Err(err).Msg("grpc force to shutdown")
	}

	// catching ctx.Done(). timeout of 5 seconds.
	<-ctx.Done()

	log.Info().Msg("Server exiting")
}
