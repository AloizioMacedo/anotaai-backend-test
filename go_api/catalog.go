package main

import (
	"context"
	"net/http"
	"time"

	"github.com/labstack/echo/v4"
	"go.mongodb.org/mongo-driver/bson"
)

type Catalog struct {
	Owner   string         `json:"owner"`
	Catalog []CatalogEntry `json:"catalog"`
}

type CatalogEntry struct {
	CategoryTitle       string               `json:"categoryTitle"`
	CategoryDescription string               `json:"categoryDescription"`
	Items               []StreamlinedProduct `json:"items"`
}
type StreamlinedProduct struct {
	Title       string `json:"title"`
	Description string `json:"description"`
	Price       int    `json:"price"`
}

func GetCatalog(c echo.Context) error {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	owner := c.QueryParam("owner")

	category_collection := CLIENT.Database(DATABASE).Collection(category_collection)
	product_collection := CLIENT.Database(DATABASE).Collection(product_collection)

	cursor, err := category_collection.Find(ctx, bson.M{"owner": owner})
	if err != nil {
		panic(err)
	}

	catalog := Catalog{Owner: owner}

	for cursor.Next(ctx) {
		var category Category

		err = cursor.Decode(&category)
		if err != nil {
			panic(err)
		}

		catalog_entry := CatalogEntry{CategoryTitle: category.Title, CategoryDescription: category.Description}

		cursor2, err := product_collection.Find(ctx, bson.M{"category": category.Title})
		if err != nil {
			panic(err)
		}

		for cursor2.Next(ctx) {
			var product Product

			err = cursor2.Decode(&product)
			if err != nil {
				panic(err)
			}

			catalog_entry.Items = append(catalog_entry.Items, StreamlinedProduct{Title: product.Title, Description: product.Description, Price: product.Price})
		}

		catalog.Catalog = append(catalog.Catalog, catalog_entry)
	}

	return c.JSON(http.StatusOK, catalog)
}
