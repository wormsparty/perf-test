package main

import (
	"fmt"
	"errors"
	
	"github.com/go-pg/pg/v10"
	"github.com/go-pg/pg/v10/orm"
)

type Sort struct {
	Sort string `json:"sort"`
	ColId string `json:"colId"`
}

type Filter struct {
	FilterType string `json:"filterType"`
	Type string `json:"type"`
	Filter string `json:"filter"`
}

type Request struct {
	Start int `json:"start"`
	End int `json:"end"`
	Sort []Sort `json:"sort"`
	Filter map[string]Filter `json:"filter"`
	GlobalSearch string `json:"globalSearch"`
}

func filterSortAndPage(dataset *orm.Query, request Request) (*orm.Query, error) {
	// Filter
	for key, filter := range request.Filter {
		if filter.FilterType != "text" {
			return nil, errors.New("Unsupported filter type")
		}
		
		col := pg.Ident(toSnakeCase(key))

		if filter.Type == "equals" {
			dataset = dataset.Where("? = ?", col, filter.Filter)
		} else if filter.Type == "notEquals" {
			dataset = dataset.Where("? <> ?", col, filter.Filter)
		} else if filter.Type == "contains" {
			dataset = dataset.Where("position(? in ?) > 0", filter.Filter, col)
		} else if filter.Type == "notContains" {
			dataset = dataset.Where("position(? in ?) = 0", filter.Filter, col)
		} else if filter.Type == "startsWith" {
			dataset = dataset.Where("? like ?", col, fmt.Sprintf("%s%%", filter.Filter))
		} else if filter.Type == "endsWith" {
			dataset = dataset.Where("? like ?", col, fmt.Sprintf("%%%s", filter.Filter))
		} else if filter.Type == "blank" {
			dataset = dataset.Where("(? <> '') IS NOT TRUE", col)
		} else if filter.Type == "notBlank" {
			dataset = dataset.Where("? <> ''", col)
		} else {
			return nil, errors.New("Unsupported type")
		}
	}
	
	// Global filter
	if len(request.GlobalSearch) >= 0 {
		dataset = dataset.WhereGroup(func(q *pg.Query) (*pg.Query, error) {
			for _, field := range globalSearchableFields {
				q = q.WhereOr("position(? in ?) > 0", request.GlobalSearch, pg.Ident(field))
			}
			return q, nil
		})
	}
	
	// Sort
	if len(request.Sort) > 0 {
		sort := request.Sort[0]
		dataset = dataset.Order(fmt.Sprintf("%s %s", toSnakeCase(sort.ColId), sort.Sort))
	}

	// Page
	return dataset.Offset(request.Start).Limit(request.End - request.Start), nil
}
