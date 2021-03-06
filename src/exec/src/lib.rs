extern crate futures;
extern crate grpc;
extern crate grpc_protobuf;
extern crate protobuf;
#[macro_use]
extern crate log;

pub mod occlum_exec;
pub mod occlum_exec_grpc;
pub mod server;

pub const DEFAULT_SERVER_FILE: &'static str = "occlum_exec_server";
pub const DEFAULT_CLIENT_FILE: &'static str = "occlum_exec_client";
pub const DEFAULT_SOCK_FILE: &'static str = "occlum_exec.sock";
pub const DEFAULT_SERVER_TIMER: u32 = 30;
