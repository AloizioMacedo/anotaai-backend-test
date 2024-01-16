package main

import (
	"context"
	"net/http"
	"time"

	"github.com/labstack/echo/v4"
	"go.mongodb.org/mongo-driver/bson"
)

const category_collection = "category"

type Category struct {
	Title       string `query:"title"`
	Description string `query:"description"`
	Owner       string `query:"owner"`
}

func PostCategory(c echo.Context) error {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	var category Category

	bind_err := (&echo.DefaultBinder{}).BindQueryParams(c, &category)
	if bind_err != nil {
		panic(bind_err)
	}

	collection := CLIENT.Database(DATABASE).Collection(category_collection)

	_, insert_err := collection.InsertOne(ctx, category)
	if insert_err != nil {
		return insert_err
	}

	return c.NoContent(http.StatusOK)
}

func DeleteCategory(c echo.Context) error {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	var category Category

	bind_err := (&echo.DefaultBinder{}).BindQueryParams(c, &category)
	if bind_err != nil {
		panic(bind_err)
	}

	collection := CLIENT.Database(DATABASE).Collection(category_collection)

	_, insert_err := collection.DeleteOne(ctx, bson.M{"title": category.Title})
	if insert_err != nil {
		return insert_err
	}

	return c.NoContent(http.StatusOK)
}

func CreateCategorySubRoute(group *echo.Group) {
	group.POST("", PostCategory)
	group.DELETE("/delete", DeleteCategory)
}
