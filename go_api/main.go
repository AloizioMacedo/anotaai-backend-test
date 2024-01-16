package main

import (
	"context"
	"os"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var CLIENT = MongoConnection()

const URI = "mongodb://root:example@localhost:27017"

const DATABASE = "backend"

func MongoConnection() *mongo.Client {
	serverAPI := options.ServerAPI(options.ServerAPIVersion1)
	opts := options.Client().ApplyURI(URI).SetServerAPIOptions(serverAPI)

	// Create a new client and connect to the server
	client, err := mongo.Connect(context.TODO(), opts)
	if err != nil {
		panic(err)
	}

	println(os.Stderr, "INFO: Connected to MongoDB!")

	return client
}

func main() {
	e := echo.New()

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	CreateProductSubRoute(e.Group("/product"))
	CreateCategorySubRoute(e.Group("/category"))

	e.GET("/catalog", GetCatalog)

	e.Logger.Fatal(e.Start(":8080"))
}
