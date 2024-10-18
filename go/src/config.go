package main

import (
	"os"

	"github.com/gin-gonic/gin"
	"gopkg.in/yaml.v3"
)

type Config struct {
	Server struct {
		Host         string   `yaml:"host"`
		Port         string   `yaml:"port"`
		CorsOrigins  []string `yaml:"cors_origins"`
		CorsAllowAll bool     `yaml:"cors_allow_all"`
		Mode         string   `yaml:"mode"`
	} `yaml:"server"`
	Database struct {
		Address  string `yaml:"address"`
		Username string `yaml:"user"`
		Password string `yaml:"pass"`
		Name     string `yaml:"name"`
	} `yaml:"database"`
}

func setConfig(cfg *Config) gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Set("cfg", cfg)
		c.Next()
	}
}

func readConfig(cfg *Config) {
	f, err := os.Open("config.yml")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	decoder := yaml.NewDecoder(f)
	err = decoder.Decode(cfg)

	if err != nil {
		panic(err)
	}
}
