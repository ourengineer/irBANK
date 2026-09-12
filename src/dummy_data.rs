/// irBANK - Dummy Data Generator
/// Author: Algie Bookshelves
/// Purpose: Generates realistic test data for the system

use crate::database::DatabaseService;
use crate::models::*;
use crate::security;
use std::error::Error;
use uuid::Uuid;

pub struct DummyDataGenerator {
    db: DatabaseService,
}

impl DummyDataGenerator {
    pub fn new(db: DatabaseService) -> Self {
        DummyDataGenerator { db }
    }

    /// Generate all dummy data
    pub async fn generate_all(&self) -> Result<(), Box<dyn Error>> {
        println!("🏦 Initializing irBANK Database...");
        
        // Initialize schema
        self.db.init_schema().await?;
        println!("✓ Database schema initialized");

        // Initialize account types
        self.db.init_account_types().await?;
        println!("✓ Account types initialized");

        // Generate customers and accounts
        self.generate_customers_and_accounts().await?;
        println!("✓ Customers and accounts created");

        // Generate transactions
        self.generate_transactions().await?;
        println!("✓ Transactions generated");

        // Generate intellectual properties
        self.generate_intellectual_properties().await?;
        println!("✓ Intellectual properties created");

        // Generate auctions
        self.generate_auctions().await?;
        println!("✓ Auctions created");

        println!("\n✅ irBANK Database successfully populated!");
        Ok(())
    }

    /// Generate customers and their accounts
    async fn generate_customers_and_accounts(&self) -> Result<(), Box<dyn Error>> {
        let customers_data = vec![
            ("john.smith@example.com", "555-0101", "JohnSmith@123", "John", "Smith", Some("1985-03-15")),
            ("sarah.johnson@example.com", "555-0102", "SarahJ@2024", "Sarah", "Johnson", Some("1990-07-22")),
            ("michael.chen@example.com", "555-0103", "MichaelChen#99", "Michael", "Chen", Some("1988-11-08")),
            ("emma.wilson@example.com", "555-0104", "EmmaW!2024", "Emma", "Wilson", Some("1992-05-14")),
            ("david.martinez@example.com", "555-0105", "DavidM@123", "David", "Martinez", Some("1987-09-25")),
            ("jessica.brown@example.com", "555-0106", "JessicaBrown!", "Jessica", "Brown", Some("1991-12-03")),
            ("robert.taylor@example.com", "555-0107", "RobertT#2024", "Robert", "Taylor", Some("1986-02-18")),
            ("lisa.anderson@example.com", "555-0108", "LisaA@123", "Lisa", "Anderson", Some("1989-08-30")),
        ];

        for (email, phone, password, first_name, last_name, dob) in customers_data {
            // Create customer
            let customer = self.db.create_customer(
                email,
                Some(phone),
                password,
                first_name,
                last_name,
                dob,
            ).await?;

            // Create multiple accounts for each customer
            let account_types = vec![2, 3, 4]; // Checking, Savings, Seed Capital
            for account_type_id in account_types {
                self.db.create_account(&customer.customer_id, account_type_id, None).await?;
            }
        }

        Ok(())
    }

    /// Generate sample transactions
    async fn generate_transactions(&self) -> Result<(), Box<dyn Error>> {
        let customers = vec![
            "john.smith@example.com",
            "sarah.johnson@example.com",
            "michael.chen@example.com",
            "emma.wilson@example.com",
        ];

        for email in customers {
            if let Some(customer) = self.db.get_customer_by_email(email).await? {
                if let Ok(accounts) = self.db.get_customer_accounts(&customer.customer_id).await {
                    if let Some(account) = accounts.first() {
                        // Initial deposit
                        self.db.log_transaction(
                            &account.account_id,
                            &customer.customer_id,
                            "deposit",
                            "50000.00",
                            "USD",
                            Some("0.00"),
                            Some("50000.00"),
                            Some("Initial deposit"),
                            Some("192.168.1.100"),
                            Some("United States"),
                            Some("New York"),
                            Some("NY"),
                            Some("40.7128"),
                            Some("-74.0060"),
                            Some(&security::generate_device_fingerprint("Mozilla/5.0", "192.168.1.100")),
                            Some("Mozilla/5.0"),
                        ).await?;

                        // Withdrawal
                        self.db.log_transaction(
                            &account.account_id,
                            &customer.customer_id,
                            "withdrawal",
                            "5000.00",
                            "USD",
                            Some("50000.00"),
                            Some("45000.00"),
                            Some("Withdrawal for personal use"),
                            Some("192.168.1.100"),
                            Some("United States"),
                            Some("New York"),
                            Some("NY"),
                            Some("40.7128"),
                            Some("-74.0060"),
                            Some(&security::generate_device_fingerprint("Mozilla/5.0", "192.168.1.100")),
                            Some("Mozilla/5.0"),
                        ).await?;

                        // Interest payment
                        self.db.log_transaction(
                            &account.account_id,
                            &customer.customer_id,
                            "interest",
                            "45.00",
                            "USD",
                            Some("45000.00"),
                            Some("45045.00"),
                            Some("Monthly interest payment"),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        ).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate intellectual properties
    async fn generate_intellectual_properties(&self) -> Result<(), Box<dyn Error>> {
        let ip_data = vec![
            ("john.smith@example.com", "Software Payment Gateway", "A revolutionary payment processing system", "patent"),
            ("sarah.johnson@example.com", "AI Trading Algorithm", "Advanced machine learning algorithm for stock trading", "patent"),
            ("michael.chen@example.com", "Blockchain Security Protocol", "Enhanced security protocol using blockchain technology", "software"),
            ("emma.wilson@example.com", "Financial Analytics Suite", "Comprehensive analytics tool for financial data", "copyright"),
            ("david.martinez@example.com", "Mobile Banking App Design", "User-friendly interface for mobile banking", "design"),
        ];

        for (email, title, description, ip_type) in ip_data {
            if let Some(customer) = self.db.get_customer_by_email(email).await? {
                if let Ok(accounts) = self.db.get_customer_accounts(&customer.customer_id).await {
                    if let Some(account) = accounts.first() {
                        let file_hash = security::generate_sha256_hash(title);
                        self.db.create_intellectual_property(
                            &customer.customer_id,
                            &account.account_id,
                            ip_type,
                            title,
                            Some(description),
                            &file_hash,
                            Some(&format!("/documents/ip/{}/document.pdf", Uuid::new_v4())),
                        ).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate auctions
    async fn generate_auctions(&self) -> Result<(), Box<dyn Error>> {
        // For now, just create the structure
        // Full auction generation would require appraisals first
        println!("  → Auction data generation requires appraisal data (future implementation)");
        Ok(())
    }
}

/// Generate sample security events
pub async fn log_sample_security_events(db: &DatabaseService) -> Result<(), Box<dyn Error>> {
    if let Some(customer) = db.get_customer_by_email("john.smith@example.com").await? {
        // Successful login
        db.log_security_event(
            Some(&customer.customer_id),
            "login_success",
            "low",
            Some("192.168.1.100"),
            Some("United States"),
            Some("New York"),
            Some("NY"),
            Some("40.7128"),
            Some("-74.0060"),
            Some(&security::generate_device_fingerprint("Mozilla/5.0", "192.168.1.100")),
            Some("Mozilla/5.0"),
            Some("Successful login from known device"),
            None,
        ).await?;

        // Suspicious activity
        db.log_security_event(
            Some(&customer.customer_id),
            "suspicious_activity",
            "high",
            Some("203.0.113.45"),
            Some("Unknown Country"),
            Some("Unknown City"),
            None,
            None,
            None,
            Some(&security::generate_device_fingerprint("Unknown Agent", "203.0.113.45")),
            Some("Unknown Agent"),
            Some("Multiple failed login attempts detected"),
            Some("Account locked for 30 minutes"),
        ).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dummy_data_generation() {
        // This test would run the dummy data generator
        // and verify the database is properly populated
        println!("Test: Dummy data generation would be tested here");
    }
}
