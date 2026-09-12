-- irBANK Database Schema
-- War-Proof and Hack-Proof Intellectual Property Bank
-- Author: Algie Bookshelves
-- Created: 2026-09-12

-- =====================================================
-- CORE TABLES
-- =====================================================

-- Customers/Users Table
CREATE TABLE IF NOT EXISTS customers (
    customer_id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    phone TEXT UNIQUE,
    password_hash TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    date_of_birth DATE,
    ssn_hash TEXT,
    kyc_status TEXT CHECK(kyc_status IN ('pending', 'approved', 'rejected', 'in_review')),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    two_factor_enabled BOOLEAN DEFAULT FALSE,
    last_login TIMESTAMP,
    login_attempts INTEGER DEFAULT 0,
    last_failed_login TIMESTAMP,
    account_locked BOOLEAN DEFAULT FALSE,
    locked_until TIMESTAMP
);

-- Account Types Table
CREATE TABLE IF NOT EXISTS account_types (
    account_type_id INTEGER PRIMARY KEY,
    account_type_name TEXT UNIQUE NOT NULL,
    description TEXT,
    daily_transaction_limit DECIMAL(18, 2),
    monthly_transaction_limit DECIMAL(18, 2),
    min_balance DECIMAL(18, 2),
    interest_rate DECIMAL(5, 4),
    features TEXT -- JSON stored as TEXT
);

-- Accounts Table (One customer can have many accounts)
CREATE TABLE IF NOT EXISTS accounts (
    account_id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    account_type_id INTEGER NOT NULL,
    account_number TEXT UNIQUE NOT NULL,
    balance DECIMAL(18, 2) NOT NULL DEFAULT 0.00,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT CHECK(status IN ('active', 'inactive', 'frozen', 'closed')) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_transaction_date TIMESTAMP,
    is_primary BOOLEAN DEFAULT FALSE,
    overdraft_limit DECIMAL(18, 2) DEFAULT 0.00,
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id) ON DELETE CASCADE,
    FOREIGN KEY (account_type_id) REFERENCES account_types(account_type_id),
    UNIQUE(customer_id, account_type_id)
);

-- =====================================================
-- SECURITY AND WAR-PROOF TABLES
-- =====================================================

-- Transaction Log (War-Proof - Immutable Ledger)
CREATE TABLE IF NOT EXISTS transaction_log (
    transaction_id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    transaction_type TEXT CHECK(transaction_type IN ('deposit', 'withdrawal', 'transfer', 'payment', 'interest', 'fee', 'auction_purchase', 'auction_sale')) NOT NULL,
    amount DECIMAL(18, 2) NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    balance_before DECIMAL(18, 2),
    balance_after DECIMAL(18, 2),
    description TEXT,
    status TEXT CHECK(status IN ('pending', 'completed', 'failed', 'cancelled', 'reversed')) DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    client_ip_address TEXT,
    client_location_country TEXT,
    client_location_city TEXT,
    client_location_state TEXT,
    client_location_latitude DECIMAL(10, 8),
    client_location_longitude DECIMAL(11, 8),
    device_fingerprint TEXT,
    user_agent TEXT,
    reference_transaction_id TEXT,
    notes TEXT,
    verification_hash TEXT NOT NULL, -- SHA-256 hash for immutability verification
    FOREIGN KEY (account_id) REFERENCES accounts(account_id),
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id)
);

-- Security Events Log (Intrusion Detection)
CREATE TABLE IF NOT EXISTS security_events (
    event_id TEXT PRIMARY KEY,
    customer_id TEXT,
    event_type TEXT CHECK(event_type IN ('login_attempt', 'login_success', 'login_failure', 'password_change', 'account_access_unauthorized', 'suspicious_activity', 'rate_limit_exceeded', 'ip_change', 'device_change', 'transaction_anomaly')) NOT NULL,
    severity TEXT CHECK(severity IN ('low', 'medium', 'high', 'critical')) DEFAULT 'medium',
    ip_address TEXT,
    location_country TEXT,
    location_city TEXT,
    location_state TEXT,
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    device_fingerprint TEXT,
    user_agent TEXT,
    description TEXT,
    action_taken TEXT,
    is_resolved BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id) ON DELETE SET NULL
);

-- Access Log (War-Proof Audit Trail)
CREATE TABLE IF NOT EXISTS access_log (
    access_id TEXT PRIMARY KEY,
    customer_id TEXT,
    account_id TEXT,
    action TEXT NOT NULL,
    resource_accessed TEXT,
    ip_address TEXT,
    location_country TEXT,
    location_city TEXT,
    location_state TEXT,
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    device_fingerprint TEXT,
    user_agent TEXT,
    status TEXT CHECK(status IN ('success', 'failure', 'denied')) DEFAULT 'success',
    reason_denied TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id) ON DELETE SET NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(account_id) ON DELETE SET NULL
);

-- =====================================================
-- INTELLECTUAL PROPERTY & SEED CAPITAL TABLES
-- =====================================================

-- Intellectual Properties Table
CREATE TABLE IF NOT EXISTS intellectual_properties (
    ip_id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    ip_type TEXT CHECK(ip_type IN ('patent', 'copyright', 'trademark', 'trade_secret', 'software', 'design', 'plant_variety', 'other')) NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    creation_date DATE,
    registration_date DATE,
    registration_number TEXT,
    registration_authority TEXT,
    file_hash TEXT NOT NULL, -- SHA-256 for document verification
    document_path TEXT,
    value_estimate DECIMAL(18, 2),
    currency TEXT DEFAULT 'USD',
    status TEXT CHECK(status IN ('pending_appraisal', 'appraised', 'listed_for_auction', 'sold', 'removed', 'disputed')) DEFAULT 'pending_appraisal',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id),
    FOREIGN KEY (account_id) REFERENCES accounts(account_id)
);

-- Appraisal Agents Table
CREATE TABLE IF NOT EXISTS appraisal_agents (
    agent_id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    phone TEXT,
    license_number TEXT UNIQUE NOT NULL,
    specializations TEXT, -- JSON array
    total_appraisals INTEGER DEFAULT 0,
    average_rating DECIMAL(3, 2),
    status TEXT CHECK(status IN ('active', 'inactive', 'suspended')) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- IP Appraisals Table
CREATE TABLE IF NOT EXISTS ip_appraisals (
    appraisal_id TEXT PRIMARY KEY,
    ip_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    appraised_value DECIMAL(18, 2) NOT NULL,
    currency TEXT DEFAULT 'USD',
    appraisal_report TEXT,
    start_bid_price DECIMAL(18, 2),
    sell_now_price DECIMAL(18, 2),
    appraisal_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    approval_status TEXT CHECK(approval_status IN ('pending', 'approved', 'rejected', 'revision_requested')) DEFAULT 'pending',
    approved_by TEXT,
    approved_at TIMESTAMP,
    notes TEXT,
    FOREIGN KEY (ip_id) REFERENCES intellectual_properties(ip_id),
    FOREIGN KEY (agent_id) REFERENCES appraisal_agents(agent_id)
);

-- =====================================================
-- AUCTION HOUSE TABLES
-- =====================================================

-- Auctions Table
CREATE TABLE IF NOT EXISTS auctions (
    auction_id TEXT PRIMARY KEY,
    ip_id TEXT NOT NULL,
    appraisal_id TEXT NOT NULL,
    seller_customer_id TEXT NOT NULL,
    start_bid_price DECIMAL(18, 2) NOT NULL,
    current_bid_price DECIMAL(18, 2) NOT NULL,
    reserve_price DECIMAL(18, 2),
    sell_now_price DECIMAL(18, 2),
    currency TEXT DEFAULT 'USD',
    auction_start TIMESTAMP NOT NULL,
    auction_end TIMESTAMP NOT NULL,
    status TEXT CHECK(status IN ('draft', 'active', 'extended', 'sold', 'unsold', 'cancelled', 'on_hold')) DEFAULT 'draft',
    highest_bidder_customer_id TEXT,
    category TEXT,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ip_id) REFERENCES intellectual_properties(ip_id),
    FOREIGN KEY (appraisal_id) REFERENCES ip_appraisals(appraisal_id),
    FOREIGN KEY (seller_customer_id) REFERENCES customers(customer_id),
    FOREIGN KEY (highest_bidder_customer_id) REFERENCES customers(customer_id) ON DELETE SET NULL
);

-- Auction Bids Table
CREATE TABLE IF NOT EXISTS auction_bids (
    bid_id TEXT PRIMARY KEY,
    auction_id TEXT NOT NULL,
    bidder_customer_id TEXT NOT NULL,
    bid_amount DECIMAL(18, 2) NOT NULL,
    currency TEXT DEFAULT 'USD',
    bid_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ip_address TEXT,
    location_country TEXT,
    location_city TEXT,
    location_state TEXT,
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    device_fingerprint TEXT,
    user_agent TEXT,
    status TEXT CHECK(status IN ('active', 'outbid', 'cancelled', 'rejected')) DEFAULT 'active',
    notes TEXT,
    FOREIGN KEY (auction_id) REFERENCES auctions(auction_id) ON DELETE CASCADE,
    FOREIGN KEY (bidder_customer_id) REFERENCES customers(customer_id)
);

-- Auction Payments Table
CREATE TABLE IF NOT EXISTS auction_payments (
    payment_id TEXT PRIMARY KEY,
    auction_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    payment_type TEXT CHECK(payment_type IN ('bid_deposit', 'full_payment', 'partial_payment', 'refund')) DEFAULT 'full_payment',
    amount DECIMAL(18, 2) NOT NULL,
    currency TEXT DEFAULT 'USD',
    payment_method TEXT CHECK(payment_method IN ('bank_transfer', 'card', 'wire', 'check', 'crypto')) DEFAULT 'bank_transfer',
    payment_status TEXT CHECK(payment_status IN ('pending', 'processing', 'completed', 'failed', 'refunded', 'cancelled')) DEFAULT 'pending',
    transaction_id TEXT UNIQUE,
    reference_number TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMP,
    FOREIGN KEY (auction_id) REFERENCES auctions(auction_id),
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id),
    FOREIGN KEY (account_id) REFERENCES accounts(account_id)
);

-- =====================================================
-- SPECIAL ACCOUNT TYPES TABLES
-- =====================================================

-- World Brokerage Accounts
CREATE TABLE IF NOT EXISTS world_brokerage_details (
    brokerage_id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL UNIQUE,
    broker_license_number TEXT,
    trading_enabled BOOLEAN DEFAULT TRUE,
    max_leverage DECIMAL(5, 2),
    available_markets TEXT, -- JSON array
    portfolio_value DECIMAL(18, 2),
    unrealized_gains DECIMAL(18, 2),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_id) REFERENCES accounts(account_id)
);

-- Currency Exchange Accounts
CREATE TABLE IF NOT EXISTS currency_exchange_details (
    exchange_id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL UNIQUE,
    base_currency TEXT DEFAULT 'USD',
    exchange_enabled BOOLEAN DEFAULT TRUE,
    exchange_rate_update_frequency TEXT DEFAULT 'real-time',
    total_volume_exchanged DECIMAL(18, 2),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_id) REFERENCES accounts(account_id)
);

-- Business Accounts
CREATE TABLE IF NOT EXISTS business_account_details (
    business_id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL UNIQUE,
    business_name TEXT NOT NULL,
    business_registration_number TEXT NOT NULL,
    business_type TEXT,
    industry TEXT,
    employee_count INTEGER,
    annual_revenue DECIMAL(18, 2),
    tax_id TEXT,
    authorized_signatories INTEGER DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_id) REFERENCES accounts(account_id)
);

-- Seed Capital Details
CREATE TABLE IF NOT EXISTS seed_capital_details (
    seed_id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL UNIQUE,
    investment_stage TEXT CHECK(investment_stage IN ('pre_seed', 'seed', 'seed_a', 'seed_b')),
    target_raise_amount DECIMAL(18, 2),
    current_raise_amount DECIMAL(18, 2) DEFAULT 0,
    investor_count INTEGER DEFAULT 0,
    upload_portal_enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_id) REFERENCES accounts(account_id)
);

-- =====================════
-- CURRENCY EXCHANGE TRANSACTIONS
-- =====================================================

CREATE TABLE IF NOT EXISTS currency_exchanges (
    exchange_transaction_id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    from_currency TEXT NOT NULL,
    to_currency TEXT NOT NULL,
    from_amount DECIMAL(18, 2) NOT NULL,
    to_amount DECIMAL(18, 2) NOT NULL,
    exchange_rate DECIMAL(18, 8) NOT NULL,
    fee_amount DECIMAL(18, 2),
    fee_currency TEXT,
    exchange_status TEXT CHECK(exchange_status IN ('pending', 'completed', 'failed', 'cancelled')) DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    ip_address TEXT,
    location_country TEXT,
    location_city TEXT,
    location_state TEXT,
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    FOREIGN KEY (account_id) REFERENCES accounts(account_id),
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id)
);

-- =====================================================
-- DOCUMENT UPLOAD & VERIFICATION TABLES
-- =====================================================

CREATE TABLE IF NOT EXISTS document_uploads (
    document_id TEXT PRIMARY KEY,
    ip_id TEXT,
    customer_id TEXT NOT NULL,
    document_type TEXT CHECK(document_type IN ('patent', 'copyright_cert', 'trademark_cert', 'technical_specs', 'business_plan', 'financial_statements', 'identification', 'proof_of_ownership', 'other')) NOT NULL,
    original_filename TEXT,
    file_path TEXT NOT NULL,
    file_size INTEGER,
    file_hash TEXT NOT NULL, -- SHA-256
    mime_type TEXT,
    upload_status TEXT CHECK(upload_status IN ('pending_verification', 'verified', 'rejected', 'quarantined')) DEFAULT 'pending_verification',
    virus_scan_status TEXT CHECK(virus_scan_status IN ('clean', 'infected', 'suspicious', 'pending')) DEFAULT 'pending',
    virus_scan_date TIMESTAMP,
    verified_by TEXT,
    verification_notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    verified_at TIMESTAMP,
    FOREIGN KEY (ip_id) REFERENCES intellectual_properties(ip_id) ON DELETE SET NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id)
);

-- =====================================================
-- INDEXES FOR PERFORMANCE & SECURITY
-- =====================================================

CREATE INDEX idx_customers_email ON customers(email);
CREATE INDEX idx_customers_created_at ON customers(created_at);
CREATE INDEX idx_customers_account_locked ON customers(account_locked);

CREATE INDEX idx_accounts_customer_id ON accounts(customer_id);
CREATE INDEX idx_accounts_account_type_id ON accounts(account_type_id);
CREATE INDEX idx_accounts_status ON accounts(status);

CREATE INDEX idx_transaction_log_account_id ON transaction_log(account_id);
CREATE INDEX idx_transaction_log_customer_id ON transaction_log(customer_id);
CREATE INDEX idx_transaction_log_created_at ON transaction_log(created_at);
CREATE INDEX idx_transaction_log_client_ip ON transaction_log(client_ip_address);
CREATE INDEX idx_transaction_log_status ON transaction_log(status);
CREATE INDEX idx_transaction_log_verification_hash ON transaction_log(verification_hash);

CREATE INDEX idx_security_events_customer_id ON security_events(customer_id);
CREATE INDEX idx_security_events_created_at ON security_events(created_at);
CREATE INDEX idx_security_events_severity ON security_events(severity);
CREATE INDEX idx_security_events_event_type ON security_events(event_type);

CREATE INDEX idx_access_log_customer_id ON access_log(customer_id);
CREATE INDEX idx_access_log_account_id ON access_log(account_id);
CREATE INDEX idx_access_log_created_at ON access_log(created_at);
CREATE INDEX idx_access_log_ip_address ON access_log(ip_address);

CREATE INDEX idx_intellectual_properties_customer_id ON intellectual_properties(customer_id);
CREATE INDEX idx_intellectual_properties_status ON intellectual_properties(status);
CREATE INDEX idx_intellectual_properties_ip_type ON intellectual_properties(ip_type);

CREATE INDEX idx_auctions_seller_customer_id ON auctions(seller_customer_id);
CREATE INDEX idx_auctions_highest_bidder ON auctions(highest_bidder_customer_id);
CREATE INDEX idx_auctions_status ON auctions(status);
CREATE INDEX idx_auctions_auction_end ON auctions(auction_end);

CREATE INDEX idx_auction_bids_auction_id ON auction_bids(auction_id);
CREATE INDEX idx_auction_bids_bidder_customer_id ON auction_bids(bidder_customer_id);
CREATE INDEX idx_auction_bids_bid_timestamp ON auction_bids(bid_timestamp);

CREATE INDEX idx_auction_payments_auction_id ON auction_payments(auction_id);
CREATE INDEX idx_auction_payments_customer_id ON auction_payments(customer_id);
CREATE INDEX idx_auction_payments_payment_status ON auction_payments(payment_status);

-- =====================================================
-- TRIGGERS FOR DATA INTEGRITY
-- =====================================================

-- Update timestamp on customers table
CREATE TRIGGER IF NOT EXISTS trigger_update_customers_timestamp
AFTER UPDATE ON customers
BEGIN
  UPDATE customers SET updated_at = CURRENT_TIMESTAMP WHERE customer_id = NEW.customer_id;
END;

-- Update timestamp on accounts table
CREATE TRIGGER IF NOT EXISTS trigger_update_accounts_timestamp
AFTER UPDATE ON accounts
BEGIN
  UPDATE accounts SET updated_at = CURRENT_TIMESTAMP WHERE account_id = NEW.account_id;
END;

-- Update timestamp on intellectual_properties table
CREATE TRIGGER IF NOT EXISTS trigger_update_ip_timestamp
AFTER UPDATE ON intellectual_properties
BEGIN
  UPDATE intellectual_properties SET updated_at = CURRENT_TIMESTAMP WHERE ip_id = NEW.ip_id;
END;

-- Update timestamp on auctions table
CREATE TRIGGER IF NOT EXISTS trigger_update_auctions_timestamp
AFTER UPDATE ON auctions
BEGIN
  UPDATE auctions SET updated_at = CURRENT_TIMESTAMP WHERE auction_id = NEW.auction_id;
END;
