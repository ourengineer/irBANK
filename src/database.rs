/// irBANK - Database Service Layer
/// Author: Algie Bookshelves
/// Purpose: Handles all database operations

use sqlx::{sqlite::SqlitePool, Row};
use uuid::Uuid;
use crate::models::*;
use crate::security;
use std::error::Error;

pub struct DatabaseService {
    pool: SqlitePool,
}

impl DatabaseService {
    /// Create new database service instance
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(DatabaseService { pool })
    }

    /// Initialize database schema
    pub async fn init_schema(&self) -> Result<(), Box<dyn Error>> {
        let schema = include_str!("../schema/irbank.sql");
        for statement in schema.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed).execute(&self.pool).await?;
            }
        }
        Ok(())
    }

    // =====================================================
    // CUSTOMER OPERATIONS
    // =====================================================

    /// Create new customer
    pub async fn create_customer(
        &self,
        email: &str,
        phone: Option<&str>,
        password: &str,
        first_name: &str,
        last_name: &str,
        date_of_birth: Option<&str>,
    ) -> Result<Customer, Box<dyn Error>> {
        let customer_id = Uuid::new_v4().to_string();
        let password_hash = security::hash_password(password)?;
        let now = chrono::Utc::now().to_rfc3339();

        let customer = sqlx::query_as::<_, Customer>(
            "INSERT INTO customers (
                customer_id, email, phone, password_hash, first_name, last_name,
                date_of_birth, kyc_status, created_at, updated_at, is_active
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"
        )
        .bind(&customer_id)
        .bind(email)
        .bind(phone)
        .bind(&password_hash)
        .bind(first_name)
        .bind(last_name)
        .bind(date_of_birth)
        .bind("pending")
        .bind(&now)
        .bind(&now)
        .bind(true)
        .fetch_one(&self.pool)
        .await?;

        Ok(customer)
    }

    /// Get customer by email
    pub async fn get_customer_by_email(&self, email: &str) -> Result<Option<Customer>, Box<dyn Error>> {
        let customer = sqlx::query_as::<_, Customer>(
            "SELECT * FROM customers WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(customer)
    }

    /// Get customer by ID
    pub async fn get_customer_by_id(&self, customer_id: &str) -> Result<Option<Customer>, Box<dyn Error>> {
        let customer = sqlx::query_as::<_, Customer>(
            "SELECT * FROM customers WHERE customer_id = ?"
        )
        .bind(customer_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(customer)
    }

    // =====================================================
    // ACCOUNT OPERATIONS
    // =====================================================

    /// Create new account
    pub async fn create_account(
        &self,
        customer_id: &str,
        account_type_id: i32,
        currency: Option<&str>,
    ) -> Result<Account, Box<dyn Error>> {
        let account_id = Uuid::new_v4().to_string();
        let account_number = format!("{}{}", IRBANK_ROUTING_NUMBER, Uuid::new_v4().to_string()[0..8].to_uppercase());
        let now = chrono::Utc::now().to_rfc3339();
        let curr = currency.unwrap_or("USD");

        let account = sqlx::query_as::<_, Account>(
            "INSERT INTO accounts (
                account_id, customer_id, account_type_id, account_number, balance,
                currency, status, created_at, updated_at, routing_number, bank_name
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"
        )
        .bind(&account_id)
        .bind(customer_id)
        .bind(account_type_id)
        .bind(&account_number)
        .bind("0.00")
        .bind(curr)
        .bind("active")
        .bind(&now)
        .bind(&now)
        .bind(IRBANK_ROUTING_NUMBER)
        .bind(IRBANK_NAME)
        .fetch_one(&self.pool)
        .await?;

        Ok(account)
    }

    /// Get accounts by customer
    pub async fn get_customer_accounts(&self, customer_id: &str) -> Result<Vec<Account>, Box<dyn Error>> {
        let accounts = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE customer_id = ? ORDER BY created_at DESC"
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(accounts)
    }

    /// Get account by ID
    pub async fn get_account_by_id(&self, account_id: &str) -> Result<Option<Account>, Box<dyn Error>> {
        let account = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE account_id = ?"
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }

    // =====================================================
    // TRANSACTION OPERATIONS (War-Proof Logging)
    // =====================================================

    /// Log transaction (Immutable)
    pub async fn log_transaction(
        &self,
        account_id: &str,
        customer_id: &str,
        transaction_type: &str,
        amount: &str,
        currency: &str,
        balance_before: Option<&str>,
        balance_after: Option<&str>,
        description: Option<&str>,
        ip_address: Option<&str>,
        country: Option<&str>,
        city: Option<&str>,
        state: Option<&str>,
        latitude: Option<&str>,
        longitude: Option<&str>,
        device_fingerprint: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<TransactionLog, Box<dyn Error>> {
        let transaction_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        
        // Generate verification hash for immutability
        let verification_hash = security::generate_transaction_hash(
            &transaction_id,
            amount,
            account_id,
            &now,
        );

        let transaction = sqlx::query_as::<_, TransactionLog>(
            "INSERT INTO transaction_log (
                transaction_id, account_id, customer_id, transaction_type, amount,
                currency, balance_before, balance_after, description, status,
                created_at, client_ip_address, client_location_country, client_location_city,
                client_location_state, client_location_latitude, client_location_longitude,
                device_fingerprint, user_agent, verification_hash
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"
        )
        .bind(&transaction_id)
        .bind(account_id)
        .bind(customer_id)
        .bind(transaction_type)
        .bind(amount)
        .bind(currency)
        .bind(balance_before)
        .bind(balance_after)
        .bind(description)
        .bind("completed")
        .bind(&now)
        .bind(ip_address)
        .bind(country)
        .bind(city)
        .bind(state)
        .bind(latitude)
        .bind(longitude)
        .bind(device_fingerprint)
        .bind(user_agent)
        .bind(&verification_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(transaction)
    }

    /// Get transactions by account
    pub async fn get_account_transactions(&self, account_id: &str, limit: i32) -> Result<Vec<TransactionLog>, Box<dyn Error>> {
        let transactions = sqlx::query_as::<_, TransactionLog>(
            "SELECT * FROM transaction_log WHERE account_id = ? ORDER BY created_at DESC LIMIT ?"
        )
        .bind(account_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }

    // =====================================================
    // SECURITY EVENT LOGGING
    // =====================================================

    /// Log security event
    pub async fn log_security_event(
        &self,
        customer_id: Option<&str>,
        event_type: &str,
        severity: &str,
        ip_address: Option<&str>,
        country: Option<&str>,
        city: Option<&str>,
        state: Option<&str>,
        latitude: Option<&str>,
        longitude: Option<&str>,
        device_fingerprint: Option<&str>,
        user_agent: Option<&str>,
        description: Option<&str>,
        action_taken: Option<&str>,
    ) -> Result<SecurityEvent, Box<dyn Error>> {
        let event_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let event = sqlx::query_as::<_, SecurityEvent>(
            "INSERT INTO security_events (
                event_id, customer_id, event_type, severity, ip_address,
                location_country, location_city, location_state, latitude, longitude,
                device_fingerprint, user_agent, description, action_taken, created_at, is_resolved
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"
        )
        .bind(&event_id)
        .bind(customer_id)
        .bind(event_type)
        .bind(severity)
        .bind(ip_address)
        .bind(country)
        .bind(city)
        .bind(state)
        .bind(latitude)
        .bind(longitude)
        .bind(device_fingerprint)
        .bind(user_agent)
        .bind(description)
        .bind(action_taken)
        .bind(&now)
        .bind(false)
        .fetch_one(&self.pool)
        .await?;

        Ok(event)
    }

    // =====================================================
    // INTELLECTUAL PROPERTY OPERATIONS
    // =====================================================

    /// Create intellectual property record
    pub async fn create_intellectual_property(
        &self,
        customer_id: &str,
        account_id: &str,
        ip_type: &str,
        title: &str,
        description: Option<&str>,
        file_hash: &str,
        document_path: Option<&str>,
    ) -> Result<IntellectualProperty, Box<dyn Error>> {
        let ip_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let ip = sqlx::query_as::<_, IntellectualProperty>(
            "INSERT INTO intellectual_properties (
                ip_id, customer_id, account_id, ip_type, title, description,
                file_hash, document_path, currency, status, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"
        )
        .bind(&ip_id)
        .bind(customer_id)
        .bind(account_id)
        .bind(ip_type)
        .bind(title)
        .bind(description)
        .bind(file_hash)
        .bind(document_path)
        .bind("USD")
        .bind("pending_appraisal")
        .bind(&now)
        .bind(&now)
        .fetch_one(&self.pool)
        .await?;

        Ok(ip)
    }

    /// Get intellectual properties by customer
    pub async fn get_customer_ips(&self, customer_id: &str) -> Result<Vec<IntellectualProperty>, Box<dyn Error>> {
        let ips = sqlx::query_as::<_, IntellectualProperty>(
            "SELECT * FROM intellectual_properties WHERE customer_id = ? ORDER BY created_at DESC"
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(ips)
    }

    // =====================================================
    // AUCTION OPERATIONS
    // =====================================================

    /// Create auction
    pub async fn create_auction(
        &self,
        ip_id: &str,
        appraisal_id: &str,
        seller_customer_id: &str,
        start_bid_price: &str,
        reserve_price: Option<&str>,
        sell_now_price: Option<&str>,
        auction_start: &str,
        auction_end: &str,
        category: Option<&str>,
    ) -> Result<Auction, Box<dyn Error>> {
        let auction_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let auction = sqlx::query_as::<_, Auction>(
            "INSERT INTO auctions (
                auction_id, ip_id, appraisal_id, seller_customer_id, start_bid_price,
                current_bid_price, reserve_price, sell_now_price, currency, auction_start,
                auction_end, status, category, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"
        )
        .bind(&auction_id)
        .bind(ip_id)
        .bind(appraisal_id)
        .bind(seller_customer_id)
        .bind(start_bid_price)
        .bind(start_bid_price)
        .bind(reserve_price)
        .bind(sell_now_price)
        .bind("USD")
        .bind(auction_start)
        .bind(auction_end)
        .bind("draft")
        .bind(category)
        .bind(&now)
        .bind(&now)
        .fetch_one(&self.pool)
        .await?;

        Ok(auction)
    }

    /// Place bid on auction
    pub async fn place_bid(
        &self,
        auction_id: &str,
        bidder_customer_id: &str,
        bid_amount: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<AuctionBid, Box<dyn Error>> {
        let bid_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let bid = sqlx::query_as::<_, AuctionBid>(
            "INSERT INTO auction_bids (
                bid_id, auction_id, bidder_customer_id, bid_amount, currency,
                bid_timestamp, ip_address, user_agent, status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"
        )
        .bind(&bid_id)
        .bind(auction_id)
        .bind(bidder_customer_id)
        .bind(bid_amount)
        .bind("USD")
        .bind(&now)
        .bind(ip_address)
        .bind(user_agent)
        .bind("active")
        .fetch_one(&self.pool)
        .await?;

        // Update auction current bid price
        sqlx::query(
            "UPDATE auctions SET current_bid_price = ?, highest_bidder_customer_id = ? WHERE auction_id = ?"
        )
        .bind(bid_amount)
        .bind(bidder_customer_id)
        .bind(auction_id)
        .execute(&self.pool)
        .await?;

        Ok(bid)
    }

    /// Get active auctions
    pub async fn get_active_auctions(&self, limit: i32) -> Result<Vec<Auction>, Box<dyn Error>> {
        let auctions = sqlx::query_as::<_, Auction>(
            "SELECT * FROM auctions WHERE status IN ('active', 'extended') ORDER BY auction_end ASC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(auctions)
    }

    /// Get auction by ID
    pub async fn get_auction_by_id(&self, auction_id: &str) -> Result<Option<Auction>, Box<dyn Error>> {
        let auction = sqlx::query_as::<_, Auction>(
            "SELECT * FROM auctions WHERE auction_id = ?"
        )
        .bind(auction_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(auction)
    }

    // =====================================================
    // ACCOUNT TYPE OPERATIONS
    // =====================================================

    /// Initialize account types
    pub async fn init_account_types(&self) -> Result<(), Box<dyn Error>> {
        let account_types = vec![
            (1, "Personal Account", "Individual personal banking account", "5000.00", "50000.00", "0.00", "0.0050"),
            (2, "Basic Checking Account", "Standard checking account for regular transactions", "10000.00", "100000.00", "500.00", "0.0000"),
            (3, "Basic Savings Account", "Traditional savings account with interest", "5000.00", "50000.00", "1000.00", "0.0350"),
            (4, "Seed Capital Account", "Investment account for startup funding", "50000.00", "500000.00", "10000.00", "0.0500"),
            (5, "World Brokerage Account", "International trading and brokerage account", "25000.00", "250000.00", "5000.00", "0.0200"),
            (6, "Currency Exchange Account", "Specialized account for currency exchange", "10000.00", "100000.00", "1000.00", "0.0150"),
            (7, "Business Account", "Account for business operations and management", "50000.00", "500000.00", "5000.00", "0.0100"),
        ];

        for (id, name, desc, daily, monthly, min_balance, rate) in account_types {
            sqlx::query(
                "INSERT OR IGNORE INTO account_types (
                    account_type_id, account_type_name, description,
                    daily_transaction_limit, monthly_transaction_limit,
                    min_balance, interest_rate
                ) VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(id)
            .bind(name)
            .bind(desc)
            .bind(daily)
            .bind(monthly)
            .bind(min_balance)
            .bind(rate)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
