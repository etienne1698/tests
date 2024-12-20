package main

import (
	"context"
	"log"
	"time"

	lib "github.com/etienne1698/grpc_go_test/lib/grpc"
	pb "github.com/etienne1698/grpc_go_test/protos/generated"
)

func main() {
	lib.InitClient()

	defer lib.CloseClient()

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	r, err := lib.Client.SayHello(ctx, &pb.HelloWorldRequest{})
	if err != nil {
		log.Fatalf("error calling function SayHello: %v", err)
	}

	log.Printf("Response from gRPC server's SayHello function: %s", r.GetMessage())
}
