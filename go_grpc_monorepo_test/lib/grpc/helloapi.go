package libhelloapi

import (
	"log"

	pb "github.com/etienne1698/grpc_go_test/protos/generated"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var Client pb.HelloWorldServiceClient
var Conn *grpc.ClientConn

func InitClient() {
	var err error
	Conn, err = grpc.NewClient(
		"localhost:50051",
		grpc.WithTransportCredentials(insecure.NewCredentials()),
		grpc.WithDefaultServiceConfig(`{"loadBalancingPolicy":"round_robin"}`),
	)
	if err != nil {
		log.Fatalf("Failed to Connect to gRPC server: %v", err)
	}
	Client = pb.NewHelloWorldServiceClient(Conn)
}

func CloseClient() {
	if Conn != nil {
		Conn.Close()
	}
}
