package main

import (
	"context"
	"net/http"
	"time"

	"github.com/labstack/echo/v4"
	"go.mongodb.org/mongo-driver/bson"
)

const product_collection = "product"

type Product struct {
	Title       string `query:"title"`
	Description string `query:"description"`
	Category    string `query:"category"`
	Owner       string `query:"owner"`
	Price       int    `query:"price"`
}

func PostProduct(c echo.Context) error {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	var product Product

	bind_err := (&echo.DefaultBinder{}).BindQueryParams(c, &product)
	if bind_err != nil {
		panic(bind_err)
	}

	collection := CLIENT.Database(DATABASE).Collection(product_collection)

	_, insert_err := collection.InsertOne(ctx, product)
	if insert_err != nil {
		return insert_err
	}

	return c.NoContent(http.StatusOK)
}

func DeleteProduct(c echo.Context) error {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	var product Product

	bind_err := (&echo.DefaultBinder{}).BindQueryParams(c, &product)
	if bind_err != nil {
		panic(bind_err)
	}

	collection := CLIENT.Database(DATABASE).Collection(product_collection)

	_, insert_err := collection.DeleteOne(ctx, bson.M{"title": product.Title})
	if insert_err != nil {
		return insert_err
	}

	return c.NoContent(http.StatusOK)
}

type Association struct {
	Product  string `query:"product"`
	Category string `query:"category"`
}

func AssociateProduct(c echo.Context) error {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	var product Association

	bind_err := (&echo.DefaultBinder{}).BindQueryParams(c, &product)
	if bind_err != nil {
		panic(bind_err)
	}

	product_collection := CLIENT.Database(DATABASE).Collection(product_collection)

	filter := bson.M{"title": product.Product}
	update := bson.M{"$set": bson.M{"category": product.Category}}

	_ = product_collection.FindOneAndUpdate(ctx, filter, update)

	return c.NoContent(http.StatusOK)
}
