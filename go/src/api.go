package main

import (
    	"net/http"

    	"github.com/gin-gonic/gin"
	"github.com/go-pg/pg/v10"
)

type Result struct {
	Data []Table `json:"data"`
	Total int `json:"total"`
}

type Login struct {
    Username string `json:"username"`
}

type Table struct {
    tableName struct{} `pg:"entity"`
    Id int64 `json:"id"`
    Colonne_1 string `json:"colonne_1"`
    Colonne_2 string `json:"colonne_2"`
}

var globalSearchableFields = [...]string{
    "colonne_1",
    "colonne_2",
}

func list(c *gin.Context) {
	var request Request
	err := c.BindJSON(&request)

	if err != nil {
       		c.AbortWithError(500, err)
		return
	}
	
	cfg := c.MustGet("cfg").(*Config)
	
	db := pg.Connect(&pg.Options{
	    Addr:     cfg.Database.Address,
	    User:     cfg.Database.Username,
	    Password: cfg.Database.Password,
	    Database: cfg.Database.Name,
	})
	defer db.Close()
	
	var entities []Table
	dataset := db.Model(&entities).Column("*")
		
	dataset, err = filterSortAndPage(dataset, request)
	
	if err != nil {
		c.AbortWithError(500, err)
		return
	}
	
	// Making the actual query
	total, err := dataset.SelectAndCount()
	
	if err != nil {
		c.AbortWithError(500, err)
		return
	}
	
	var result Result
	result.Data = entities
	result.Total = total
	
	c.JSON(http.StatusOK, result)
}

