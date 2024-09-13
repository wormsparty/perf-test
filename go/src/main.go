package main

import (
	"time"
	"fmt"
	
    	"github.com/gin-gonic/gin"
	"github.com/gin-contrib/cors"
)

func main() {
	var cfg Config
	readConfig(&cfg)

	if cfg.Server.Mode == "release" {
		gin.SetMode(gin.ReleaseMode)
	}
	
	router := gin.New()

	router.Use(
		//gin.LoggerWithWriter(gin.DefaultWriter, "/logs/"),
		gin.Recovery(),
	)

	router.Use(setConfig(&cfg))

	router.Use(cors.New(cors.Config{
	    AllowOrigins:     cfg.Server.CorsOrigins,
	    AllowAllOrigins:  cfg.Server.CorsAllowAll,
	    AllowMethods:     []string{"GET", "POST"},
	    AllowHeaders:     []string{"Origin"},
	    ExposeHeaders:    []string{"Content-Length"},
	    AllowCredentials: true,
	    MaxAge: 12 * time.Hour,
	}))
  
	router.POST("/api/list", list)

	router.Run(fmt.Sprintf("%s:%s", cfg.Server.Host, cfg.Server.Port))
}
