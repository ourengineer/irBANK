/// irBANK - Data Models
/// Author: Algie Bookshelves
/// Purpose: Defines all data structures for the system

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

// Bank Routing Information
pub const IRBANK_ROUTING_NUMBER: &str = "026013676";
pub const IRBANK_NAME: &str = "Intellectual Property Bank";
pub const IRBANK_HEADQUARTERS: &str = "Global Digital Banking";

/// Customer Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Customer {
    pub customer_id: String,
    pub email: String,
    pub phone: Option<String>,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<String>,
    pub ssn_hash: Option<String>,
    pub kyc_status: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_active: bool,
    pub two_factor_enabled: bool,
    pub last_login: Option<String>,
    pub login_attempts: i32,
    pub last_failed_login: Option<String>,
    pub account_locked: bool,
    pub locked_until: Option<String>,
}

/// Account Type Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccountType {
    pub account_type_id: i32,
    pub account_type_name: String,
    pub description: Option<String>,
    pub daily_transaction_limit: Option<String>,
    pub monthly_transaction_limit: Option<String>,
    pub min_balance: Option<String>,
    pub interest_rate: Option<String>,
    pub features: Option<String>,
}

/// Account Model with Routing Number
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub account_id: String,
    pub customer_id: String,
    pub account_type_id: i32,
    pub account_number: String,
    pub balance: String,
    pub currency: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_transaction_date: Option<String>,
    pub is_primary: bool,
    pub overdraft_limit: Option<String>,
    pub routing_number: String,
    pub bank_name: String,
}

impl Account {
    pub fn new(
        account_id: String,
        customer_id: String,
        account_type_id: i32,
        account_number: String,
    ) -> Self {
        Account {
            account_id,
            customer_id,
            account_type_id,
            account_number,
            balance: "0.00".to_string(),
            currency: "USD".to_string(),
            status: "active".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            last_transaction_date: None,
            is_primary: false,
            overdraft_limit: Some("0.00".to_string()),
            routing_number: IRBANK_ROUTING_NUMBER.to_string(),
            bank_name: IRBANK_NAME.to_string(),
        }
    }
}

/// Transaction Log Model (War-Proof, Immutable)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransactionLog {
    pub transaction_id: String,
    pub account_id: String,
    pub customer_id: String,
    pub transaction_type: String,
    pub amount: String,
    pub currency: String,
    pub balance_before: Option<String>,
    pub balance_after: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub client_ip_address: Option<String>,
    pub client_location_country: Option<String>,
    pub client_location_city: Option<String>,
    pub client_location_state: Option<String>,
    pub client_location_latitude: Option<String>,
    pub client_location_longitude: Option<String>,
    pub device_fingerprint: Option<String>,
    pub user_agent: Option<String>,
    pub reference_transaction_id: Option<String>,
    pub notes: Option<String>,
    pub verification_hash: String,
}

/// Security Event Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SecurityEvent {
    pub event_id: String,
    pub customer_id: Option<String>,
    pub event_type: String,
    pub severity: String,
    pub ip_address: Option<String>,
    pub location_country: Option<String>,
    pub location_city: Option<String>,
    pub location_state: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub device_fingerprint: Option<String>,
    pub user_agent: Option<String>,
    pub description: Option<String>,
    pub action_taken: Option<String>,
    pub is_resolved: bool,
    pub created_at: String,
    pub resolved_at: Option<String>,
}

/// Access Log Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccessLog {
    pub access_id: String,
    pub customer_id: Option<String>,
    pub account_id: Option<String>,
    pub action: String,
    pub resource_accessed: Option<String>,
    pub ip_address: Option<String>,
    pub location_country: Option<String>,
    pub location_city: Option<String>,
    pub location_state: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub device_fingerprint: Option<String>,
    pub user_agent: Option<String>,
    pub status: String,
    pub reason_denied: Option<String>,
    pub created_at: String,
}

/// Intellectual Property Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IntellectualProperty {
    pub ip_id: String,
    pub customer_id: String,
    pub account_id: String,
    pub ip_type: String,
    pub title: String,
    pub description: Option<String>,
    pub creation_date: Option<String>,
    pub registration_date: Option<String>,
    pub registration_number: Option<String>,
    pub registration_authority: Option<String>,
    pub file_hash: String,
    pub document_path: Option<String>,
    pub value_estimate: Option<String>,
    pub currency: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Appraisal Agent Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppraisalAgent {
    pub agent_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub license_number: String,
    pub specializations: Option<String>,
    pub total_appraisals: i32,
    pub average_rating: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// IP Appraisal Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IPAppraisal {
    pub appraisal_id: String,
    pub ip_id: String,
    pub agent_id: String,
    pub appraised_value: String,
    pub currency: String,
    pub appraisal_report: Option<String>,
    pub start_bid_price: Option<String>,
    pub sell_now_price: Option<String>,
    pub appraisal_date: String,
    pub approval_status: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub notes: Option<String>,
}

/// Auction Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Auction {
    pub auction_id: String,
    pub ip_id: String,
    pub appraisal_id: String,
    pub seller_customer_id: String,
    pub start_bid_price: String,
    pub current_bid_price: String,
    pub reserve_price: Option<String>,
    pub sell_now_price: Option<String>,
    pub currency: String,
    pub auction_start: String,
    pub auction_end: String,
    pub status: String,
    pub highest_bidder_customer_id: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Auction Bid Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuctionBid {
    pub bid_id: String,
    pub auction_id: String,
    pub bidder_customer_id: String,
    pub bid_amount: String,
    pub currency: String,
    pub bid_timestamp: String,
    pub ip_address: Option<String>,
    pub location_country: Option<String>,
    pub location_city: Option<String>,
    pub location_state: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub device_fingerprint: Option<String>,
    pub user_agent: Option<String>,
    pub status: String,
    pub notes: Option<String>,
}

/// Auction Payment Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuctionPayment {
    pub payment_id: String,
    pub auction_id: String,
    pub customer_id: String,
    pub account_id: String,
    pub payment_type: String,
    pub amount: String,
    pub currency: String,
    pub payment_method: String,
    pub payment_status: String,
    pub transaction_id: Option<String>,
    pub reference_number: Option<String>,
    pub created_at: String,
    pub processed_at: Option<String>,
}

/// World Brokerage Details Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorldBrokerageDetails {
    pub brokerage_id: String,
    pub account_id: String,
    pub broker_license_number: Option<String>,
    pub trading_enabled: bool,
    pub max_leverage: Option<String>,
    pub available_markets: Option<String>,
    pub portfolio_value: Option<String>,
    pub unrealized_gains: Option<String>,
    pub created_at: String,
}

/// Currency Exchange Details Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CurrencyExchangeDetails {
    pub exchange_id: String,
    pub account_id: String,
    pub base_currency: String,
    pub exchange_enabled: bool,
    pub exchange_rate_update_frequency: String,
    pub total_volume_exchanged: Option<String>,
    pub created_at: String,
}

/// Business Account Details Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BusinessAccountDetails {
    pub business_id: String,
    pub account_id: String,
    pub business_name: String,
    pub business_registration_number: String,
    pub business_type: Option<String>,
    pub industry: Option<String>,
    pub employee_count: Option<i32>,
    pub annual_revenue: Option<String>,
    pub tax_id: Option<String>,
    pub authorized_signatories: i32,
    pub created_at: String,
}

/// Seed Capital Details Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SeedCapitalDetails {
    pub seed_id: String,
    pub account_id: String,
    pub investment_stage: Option<String>,
    pub target_raise_amount: Option<String>,
    pub current_raise_amount: Option<String>,
    pub investor_count: i32,
    pub upload_portal_enabled: bool,
    pub created_at: String,
}

/// Currency Exchange Transaction Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CurrencyExchange {
    pub exchange_transaction_id: String,
    pub account_id: String,
    pub customer_id: String,
    pub from_currency: String,
    pub to_currency: String,
    pub from_amount: String,
    pub to_amount: String,
    pub exchange_rate: String,
    pub fee_amount: Option<String>,
    pub fee_currency: Option<String>,
    pub exchange_status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub ip_address: Option<String>,
    pub location_country: Option<String>,
    pub location_city: Option<String>,
    pub location_state: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

/// Document Upload Model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DocumentUpload {
    pub document_id: String,
    pub ip_id: Option<String>,
    pub customer_id: String,
    pub document_type: String,
    pub original_filename: Option<String>,
    pub file_path: String,
    pub file_size: Option<i32>,
    pub file_hash: String,
    pub mime_type: Option<String>,
    pub upload_status: String,
    pub virus_scan_status: String,
    pub virus_scan_date: Option<String>,
    pub verified_by: Option<String>,
    pub verification_notes: Option<String>,
    pub created_at: String,
    pub verified_at: Option<String>,
}

/// Bank Information Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankInfo {
    pub routing_number: String,
    pub bank_name: String,
    pub headquarters: String,
}

impl BankInfo {
    pub fn irbank() -> Self {
        BankInfo {
            routing_number: IRBANK_ROUTING_NUMBER.to_string(),
            bank_name: IRBANK_NAME.to_string(),
            headquarters: IRBANK_HEADQUARTERS.to_string(),
        }
    }
}

/// Request Models for API endpoints

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub customer_id: String,
    pub account_type_id: i32,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub account_id: String,
    pub transaction_type: String,
    pub amount: String,
    pub description: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadIPRequest {
    pub customer_id: String,
    pub account_id: String,
    pub ip_type: String,
    pub title: String,
    pub description: Option<String>,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAuctionRequest {
    pub ip_id: String,
    pub appraisal_id: String,
    pub start_bid_price: String,
    pub reserve_price: Option<String>,
    pub sell_now_price: Option<String>,
    pub auction_start: String,
    pub auction_end: String,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceBidRequest {
    pub auction_id: String,
    pub bidder_customer_id: String,
    pub bid_amount: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAuctionPaymentRequest {
    pub auction_id: String,
    pub customer_id: String,
    pub account_id: String,
    pub amount: String,
    pub payment_method: String,
}

/// Response Models

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub error_code: String,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(message: String, data: T) -> Self {
        ApiResponse {
            success: true,
            message,
            data: Some(data),
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message,
            data: None,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}
