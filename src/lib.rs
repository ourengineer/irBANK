/// irBANK - Intellectual Property Bank
/// A War-Proof and Hack-Proof Banking System
/// Author: Algie Bookshelves
/// Version: 1.0.0

pub mod models;
pub mod security;
pub mod database;
pub mod dummy_data;

pub use models::*;
pub use security::*;
pub use database::DatabaseService;
pub use dummy_data::DummyDataGenerator;

pub const VERSION: &str = "1.0.0";
pub const BANK_NAME: &str = "Intellectual Property Bank";
pub const ROUTING_NUMBER: &str = "026013676";
