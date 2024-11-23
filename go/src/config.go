package main

import (
	"log"
	"os"
	"strconv"
	"strings"

	"github.com/joho/godotenv"
)

type Config struct {
	Server struct {
		CorsOrigins  []string
		CorsAllowAll bool
		Mode         string
	}
	DatabaseUrl string
}

func readConfig(cfg *Config) {
	err := godotenv.Load("../.env")

	if err != nil {
		log.Println("Failed to open ../.env, skipping")
	}

	cfg.Server.CorsOrigins = strings.Split(os.Getenv("CORS_ALLOWED_ORIGINS"), ",")
	cfg.Server.CorsAllowAll, _ = strconv.ParseBool(os.Getenv("CORS_ORIGIN_ALLOW_ALL"))

	if os.Getenv("DEBUG") == "True" {
		cfg.Server.Mode = "debug"
	} else {
		cfg.Server.Mode = "release"
	}

	cfg.DatabaseUrl = os.Getenv("DATABASE_URL")

	//configStr, _ := json.Marshal(cfg)
	//fmt.Println(string(configStr))
}
