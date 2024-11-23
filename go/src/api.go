package main

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

type EntityResult struct {
	Data  []Table `json:"data"`
	Total int     `json:"total"`
}

type LoginResult struct {
	Username string `json:"username"`
}

type Table struct {
	tableName struct{} `pg:"entity"`
	Id        int64    `json:"id"`
	Colonne_1 string   `json:"colonne_1"`
	Colonne_2 string   `json:"colonne_2"`
}

var globalSearchableFields = [...]string{
	"colonne_1",
	"colonne_2",
}

func (h *Handler) login(c *gin.Context) {
	var userIdHeader = c.Request.Header["Userid"]

	var result LoginResult

	if len(userIdHeader) > 0 {
		result.Username = userIdHeader[0]
	} else {
		result.Username = "VDL12345"
	}

	c.JSON(http.StatusOK, result)
}

func (h *Handler) list(c *gin.Context) {
	var request GridRequest
	err := c.BindJSON(&request)

	if err != nil {
		_ = c.AbortWithError(500, err)
		return
	}

	var entities []Table
	dataset := h.db.Model(&entities).Column("*")

	dataset, err = filterSortAndPage(dataset, request)

	if err != nil {
		_ = c.AbortWithError(500, err)
		return
	}

	// Making the actual query
	total, err := dataset.SelectAndCount()

	if err != nil {
		_ = c.AbortWithError(500, err)
		return
	}

	var result EntityResult
	result.Data = entities
	result.Total = total

	c.JSON(http.StatusOK, result)
}
