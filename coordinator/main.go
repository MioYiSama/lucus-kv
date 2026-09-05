package main

import (
	"context"
	"log"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"mioyi.net/lucuskv/coordinator/proto"
)

func main() {
	conn, err := grpc.NewClient("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		panic(err)
	}
	defer conn.Close()

	client := proto.NewStorageServiceClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	// 4. 发起 RPC 调用
	req := &proto.HealthRequest{}
	resp, err := client.Health(ctx, req)
	if err != nil {
		log.Fatalf("调用 SayHello 失败: %v", err)
	}
	// 5. 处理响应结果
	log.Printf("收到服务端响应: %t", resp.GetValue())

}
