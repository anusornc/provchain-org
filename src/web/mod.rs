//! Web interface module for Phase 2 implementation
//! Provides REST API and web server functionality

pub mod server;
pub mod handlers;
pub mod auth;
pub mod models;

pub use server::WebServer;
