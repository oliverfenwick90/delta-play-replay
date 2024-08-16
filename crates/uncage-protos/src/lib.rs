pub use prost::{self, Message};

pub mod pb {
    tonic::include_proto!("cade_api.rpc");
}
