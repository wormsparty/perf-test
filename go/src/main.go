package main

import (
	"fmt"
	"github.com/go-pg/pg/v10"
	"log"
	"net/http"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

type Handler struct {
	db *pg.DB
}

func initDb(cfg *Config) *pg.DB {
	opt, err := pg.ParseURL(cfg.DatabaseUrl)

	if err != nil {
		panic(err)
	}

	opt.PoolSize = 5
	return pg.Connect(opt)
}

func initCorsConfig(cfg *Config) cors.Config {
	var corsCfg = cors.Config{
		AllowMethods:     []string{"GET", "POST", "PUT", "OPTIONS", "DELETE"},
		AllowHeaders:     []string{"Origin", "Content-Type"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
		MaxAge:           12 * time.Hour,
	}

	if cfg.Server.CorsAllowAll {
		corsCfg.AllowAllOrigins = true
	} else {
		corsCfg.AllowOrigins = cfg.Server.CorsOrigins
	}

	return corsCfg
}

func dbErrorMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Next()

		if len(c.Errors) > 0 {
			c.JSON(http.StatusInternalServerError, gin.H{
				"errors": c.Errors.Errors(),
			})
		}
	}
}

func main() {
	var cfg Config
	readConfig(&cfg)

	if cfg.Server.Mode == "release" {
		gin.SetMode(gin.ReleaseMode)
	}

	// DB pool
	db := initDb(&cfg)
	defer db.Close()

	handler := &Handler{db: db}

	fmt.Println("Testing DB connection...")

	if err := db.Ping(db.Context()); err != nil {
		log.Fatal("Erreur de connexion: ", err)
	}

	fmt.Println("OK !")

	// Webservice configuration
	router := gin.New()

	router.Use(
		//gin.LoggerWithWriter(gin.DefaultWriter, "/logs/"),
		gin.Recovery(),
	)

	var corsCfg = initCorsConfig(&cfg)
	router.Use(cors.New(corsCfg))
	router.Use(dbErrorMiddleware())

	api := router.Group("/api")
	{
		api.GET("/login", handler.login)
		api.POST("/list", handler.list)
	}

	fmt.Printf("Ready")

	_ = router.Run("0.0.0.0:8000")
}
