// Code generated by protoc-gen-go.
// source: pd_jobpb.proto
// DO NOT EDIT!

/*
Package pd_jobpd is a generated protocol buffer package.

It is generated from these files:
	pd_jobpb.proto

It has these top-level messages:
	Job
*/
package pd_jobpd

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"
import raft_cmdpb "github.com/pingcap/kvproto/pkg/raft_cmdpb"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
const _ = proto.ProtoPackageIsVersion1

type JobStatus int32

const (
	// When pd server inserts the job into job queue, the job status is pending.
	JobStatus_Pending JobStatus = 0
	// When pd server gets the first job and before handling, it must first change
	// the job's status to running. If the job is running, we should check whether
	// the job is already finished in raft server.
	JobStatus_Running  JobStatus = 1
	JobStatus_Finished JobStatus = 2
	JobStatus_Canceled JobStatus = 3
)

var JobStatus_name = map[int32]string{
	0: "Pending",
	1: "Running",
	2: "Finished",
	3: "Canceled",
}
var JobStatus_value = map[string]int32{
	"Pending":  0,
	"Running":  1,
	"Finished": 2,
	"Canceled": 3,
}

func (x JobStatus) Enum() *JobStatus {
	p := new(JobStatus)
	*p = x
	return p
}
func (x JobStatus) String() string {
	return proto.EnumName(JobStatus_name, int32(x))
}
func (x *JobStatus) UnmarshalJSON(data []byte) error {
	value, err := proto.UnmarshalJSONEnum(JobStatus_value, data, "JobStatus")
	if err != nil {
		return err
	}
	*x = JobStatus(value)
	return nil
}
func (JobStatus) EnumDescriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

// For each conf change, split operation, we will add this into job queue,
// and in another thread, it will get first job, handle it, and then pop in cycle.
type Job struct {
	JobId            *uint64                        `protobuf:"varint,1,opt,name=job_id" json:"job_id,omitempty"`
	Status           *JobStatus                     `protobuf:"varint,2,opt,name=status,enum=pd_jobpd.JobStatus" json:"status,omitempty"`
	Request          *raft_cmdpb.RaftCommandRequest `protobuf:"bytes,3,opt,name=request" json:"request,omitempty"`
	XXX_unrecognized []byte                         `json:"-"`
}

func (m *Job) Reset()                    { *m = Job{} }
func (m *Job) String() string            { return proto.CompactTextString(m) }
func (*Job) ProtoMessage()               {}
func (*Job) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *Job) GetJobId() uint64 {
	if m != nil && m.JobId != nil {
		return *m.JobId
	}
	return 0
}

func (m *Job) GetStatus() JobStatus {
	if m != nil && m.Status != nil {
		return *m.Status
	}
	return JobStatus_Pending
}

func (m *Job) GetRequest() *raft_cmdpb.RaftCommandRequest {
	if m != nil {
		return m.Request
	}
	return nil
}

func init() {
	proto.RegisterType((*Job)(nil), "pd_jobpd.Job")
	proto.RegisterEnum("pd_jobpd.JobStatus", JobStatus_name, JobStatus_value)
}

var fileDescriptor0 = []byte{
	// 189 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0xe2, 0xe2, 0x2b, 0x48, 0x89, 0xcf,
	0xca, 0x4f, 0x2a, 0x48, 0xd2, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0xe2, 0x80, 0xf2, 0x53, 0xa4,
	0x04, 0x8a, 0x12, 0xd3, 0x4a, 0xe2, 0x93, 0x73, 0x53, 0x60, 0x72, 0x4a, 0xd9, 0x5c, 0xcc, 0x5e,
	0xf9, 0x49, 0x42, 0x7c, 0x5c, 0x6c, 0x40, 0x15, 0xf1, 0x99, 0x29, 0x12, 0x8c, 0x0a, 0x8c, 0x1a,
	0x2c, 0x42, 0xca, 0x5c, 0x6c, 0xc5, 0x25, 0x89, 0x25, 0xa5, 0xc5, 0x12, 0x4c, 0x40, 0x3e, 0x9f,
	0x91, 0xb0, 0x1e, 0xcc, 0x0c, 0x3d, 0xa0, 0xf2, 0x60, 0xb0, 0x94, 0x90, 0x3e, 0x17, 0x7b, 0x51,
	0x6a, 0x61, 0x69, 0x6a, 0x71, 0x89, 0x04, 0x33, 0x50, 0x15, 0xb7, 0x91, 0x9c, 0x1e, 0x92, 0xf9,
	0x41, 0x40, 0xa6, 0x73, 0x7e, 0x6e, 0x6e, 0x62, 0x5e, 0x4a, 0x10, 0x44, 0x95, 0x96, 0x23, 0x17,
	0x27, 0x42, 0x37, 0x37, 0x17, 0x7b, 0x40, 0x6a, 0x5e, 0x4a, 0x66, 0x5e, 0xba, 0x00, 0x03, 0x88,
	0x13, 0x54, 0x9a, 0x97, 0x07, 0xe2, 0x30, 0x0a, 0xf1, 0x70, 0x71, 0xb8, 0x65, 0xe6, 0x65, 0x16,
	0x67, 0xa4, 0xa6, 0x08, 0x30, 0x81, 0x78, 0xce, 0x89, 0x79, 0xc9, 0xa9, 0x39, 0x40, 0x1e, 0x33,
	0x20, 0x00, 0x00, 0xff, 0xff, 0xc7, 0xe6, 0x2a, 0xdc, 0xdc, 0x00, 0x00, 0x00,
}