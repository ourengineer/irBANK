/// irBANK - Main Entry Point
/// Author: Algie Bookshelves
/// Purpose: CLI application for initializing and managing irBANK

use irbank::{DatabaseService, DummyDataGenerator, dummy_data};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║         irBANK - Intellectual Property Bank               ║");
    println!("║           Routing Number: 026013676                      ║");
    println!("║         War-Proof and Hack-Proof Banking System          ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Initialize database
    println!("📋 Initializing database connection...");
    let database_url = "sqlite://irbank.db";
    let db = DatabaseService::new(database_url).await?;
    println!("✓ Database connection established\n");

    // Generate dummy data
    println!("📊 Generating dummy data...\n");
    let generator = DummyDataGenerator::new(db.clone());
    generator.generate_all().await?;

    // Log sample security events
    println!("\n🔒 Logging sample security events...");
    dummy_data::log_sample_security_events(&db).await?;
    println!("✓ Security events logged\n");

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║              ✅ System Initialization Complete            ║");
    println!("║                                                           ║");
    println!("║  Bank Name: Intellectual Property Bank                   ║");
    println!("║  Routing Number: 026013676                               ║");
    println!("║  Database: irbank.db                                     ║");
    println!("║                                                           ║");
    println!("║  The system is ready for use!                            ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    Ok(())
}
