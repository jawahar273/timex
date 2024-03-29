// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             v3.21.12
// source: proto/api/v1/glue.proto

package timex

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	Machine_Send_FullMethodName     = "/timex.Machine/Send"
	Machine_SendTest_FullMethodName = "/timex.Machine/SendTest"
)

// MachineClient is the client API for Machine service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type MachineClient interface {
	Send(ctx context.Context, in *DetailRequest, opts ...grpc.CallOption) (*DetailResponse, error)
	SendTest(ctx context.Context, in *DetailResponse, opts ...grpc.CallOption) (*DetailResponse, error)
}

type machineClient struct {
	cc grpc.ClientConnInterface
}

func NewMachineClient(cc grpc.ClientConnInterface) MachineClient {
	return &machineClient{cc}
}

func (c *machineClient) Send(ctx context.Context, in *DetailRequest, opts ...grpc.CallOption) (*DetailResponse, error) {
	out := new(DetailResponse)
	err := c.cc.Invoke(ctx, Machine_Send_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *machineClient) SendTest(ctx context.Context, in *DetailResponse, opts ...grpc.CallOption) (*DetailResponse, error) {
	out := new(DetailResponse)
	err := c.cc.Invoke(ctx, Machine_SendTest_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// MachineServer is the server API for Machine service.
// All implementations must embed UnimplementedMachineServer
// for forward compatibility
type MachineServer interface {
	Send(context.Context, *DetailRequest) (*DetailResponse, error)
	SendTest(context.Context, *DetailResponse) (*DetailResponse, error)
	mustEmbedUnimplementedMachineServer()
}

// UnimplementedMachineServer must be embedded to have forward compatible implementations.
type UnimplementedMachineServer struct {
}

func (UnimplementedMachineServer) Send(context.Context, *DetailRequest) (*DetailResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Send not implemented")
}
func (UnimplementedMachineServer) SendTest(context.Context, *DetailResponse) (*DetailResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SendTest not implemented")
}
func (UnimplementedMachineServer) mustEmbedUnimplementedMachineServer() {}

// UnsafeMachineServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to MachineServer will
// result in compilation errors.
type UnsafeMachineServer interface {
	mustEmbedUnimplementedMachineServer()
}

func RegisterMachineServer(s grpc.ServiceRegistrar, srv MachineServer) {
	s.RegisterService(&Machine_ServiceDesc, srv)
}

func _Machine_Send_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(DetailRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MachineServer).Send(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Machine_Send_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MachineServer).Send(ctx, req.(*DetailRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Machine_SendTest_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(DetailResponse)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MachineServer).SendTest(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Machine_SendTest_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MachineServer).SendTest(ctx, req.(*DetailResponse))
	}
	return interceptor(ctx, in, info, handler)
}

// Machine_ServiceDesc is the grpc.ServiceDesc for Machine service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Machine_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "timex.Machine",
	HandlerType: (*MachineServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Send",
			Handler:    _Machine_Send_Handler,
		},
		{
			MethodName: "SendTest",
			Handler:    _Machine_SendTest_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "proto/api/v1/glue.proto",
}
