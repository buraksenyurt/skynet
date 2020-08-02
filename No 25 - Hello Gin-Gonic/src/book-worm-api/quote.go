package main

import "go.mongodb.org/mongo-driver/bson/primitive"

/*
	quotation yapısı mongodb'deki dokümanın GO tarafındaki izdüşümü.
	ID, mongodb tarafı için bson'un primitive tiplerinden birisi ile ifade edilebilir
*/
type quote struct {
	ID          primitive.ObjectID
	Description string
	Writer      string
	Book        string
}
