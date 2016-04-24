//#![feature(plugin)]
//#![plugin(mod_path)]

extern crate protobuf;

//mod_path! grSim_Commands (concat!(env!("OUT_DIR"), "grSim_Commands.rs"));
//mod_path! grSim_Packet (concat!(env!("OUT_DIR"), "grSim_Packet.rs"));
//mod_path! grSim_Replacement (concat!(env!("OUT_DIR"), "grSim_Replacement.rs"));
//mod_path! messages_robocup_ssl_detection (concat!(env!("OUT_DIR"), "messages_robocup_ssl_detection.rs"));
//mod_path! messages_robocup_ssl_geometry (concat!(env!("OUT_DIR"), "messages_robocup_ssl_geometry.rs"));
//mod_path! messages_robocup_ssl_refbox_log (concat!(env!("OUT_DIR"), "messages_robocup_ssl_refbox_log.rs"));
//mod_path! messages_robocup_ssl_wrapper (concat!(env!("OUT_DIR"), "messages_robocup_ssl_wrapper.rs"));
//mod_path! referee (concat!(env!("OUT_DIR"), "referee.rs"));

pub mod grSim_Commands;
pub mod grSim_Packet;
pub mod grSim_Replacement;
pub mod messages_robocup_ssl_detection;
pub mod messages_robocup_ssl_geometry;
pub mod messages_robocup_ssl_geometry_legacy;
pub mod messages_robocup_ssl_refbox_log;
pub mod messages_robocup_ssl_wrapper;
pub mod messages_robocup_ssl_wrapper_legacy;
pub mod referee;

pub use protobuf::parse_from_bytes;
pub use protobuf::core::Message;
