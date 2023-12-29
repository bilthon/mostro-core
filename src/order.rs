use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_crud::SqlxCrud;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Orders can be only Buy or Sell
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Buy,
    Sell,
}

impl FromStr for Kind {
    type Err = ();

    fn from_str(kind: &str) -> std::result::Result<Self, Self::Err> {
        match kind {
            "Buy" => std::result::Result::Ok(Self::Buy),
            "Sell" => std::result::Result::Ok(Self::Sell),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Each status that an order can have
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Canceled,
    CanceledByAdmin,
    SettledByAdmin,
    CompletedByAdmin,
    Dispute,
    Expired,
    FiatSent,
    SettledHoldInvoice,
    Pending,
    Success,
    WaitingBuyerInvoice,
    WaitingPayment,
    CooperativelyCanceled,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Active" => std::result::Result::Ok(Self::Active),
            "Canceled" => std::result::Result::Ok(Self::Canceled),
            "CanceledByAdmin" => std::result::Result::Ok(Self::CanceledByAdmin),
            "SettledByAdmin" => std::result::Result::Ok(Self::SettledByAdmin),
            "CompletedByAdmin" => std::result::Result::Ok(Self::CompletedByAdmin),
            "Dispute" => std::result::Result::Ok(Self::Dispute),
            "Expired" => std::result::Result::Ok(Self::Expired),
            "FiatSent" => std::result::Result::Ok(Self::FiatSent),
            "SettledHoldInvoice" => std::result::Result::Ok(Self::SettledHoldInvoice),
            "Pending" => std::result::Result::Ok(Self::Pending),
            "Success" => std::result::Result::Ok(Self::Success),
            "WaitingBuyerInvoice" => std::result::Result::Ok(Self::WaitingBuyerInvoice),
            "WaitingPayment" => std::result::Result::Ok(Self::WaitingPayment),
            "CooperativelyCanceled" => std::result::Result::Ok(Self::CooperativelyCanceled),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Database representation of an order
#[derive(Debug, Default, FromRow, SqlxCrud, Deserialize, Serialize, Clone)]
#[external_id]
pub struct Order {
    pub id: Uuid,
    pub kind: String,
    pub event_id: String,
    pub hash: Option<String>,
    pub preimage: Option<String>,
    pub creator_pubkey: String,
    pub cancel_initiator_pubkey: Option<String>,
    pub buyer_pubkey: Option<String>,
    pub master_buyer_pubkey: Option<String>,
    pub seller_pubkey: Option<String>,
    pub master_seller_pubkey: Option<String>,
    pub status: String,
    pub price_from_api: bool,
    pub premium: i64,
    pub payment_method: String,
    pub amount: i64,
    pub min_amount: i64,
    pub max_amount: i64,
    pub buyer_dispute: bool,
    pub seller_dispute: bool,
    pub buyer_cooperativecancel: bool,
    pub seller_cooperativecancel: bool,
    pub fee: i64,
    pub routing_fee: i64,
    pub fiat_code: String,
    pub fiat_amount: i64,
    pub buyer_invoice: Option<String>,
    pub range_parent_id: Option<Uuid>,
    pub invoice_held_at: i64,
    pub taken_at: i64,
    pub created_at: i64,
    pub buyer_sent_rate: bool,
    pub seller_sent_rate: bool,
}

impl Order {
    pub fn as_new_order(&self) -> SmallOrder {
        SmallOrder::new(
            Some(self.id),
            Some(Kind::from_str(&self.kind).unwrap()),
            Some(Status::from_str(&self.status).unwrap()),
            self.amount,
            self.fiat_code.clone(),
            self.fiat_amount,
            self.payment_method.clone(),
            self.premium,
            None,
            None,
            self.buyer_invoice.clone(),
            Some(self.created_at),
            None,
            None,
        )
    }
}

/// We use this struct to create a new order
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct SmallOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub kind: Option<Kind>,
    pub status: Option<Status>,
    pub amount: i64,
    pub fiat_code: String,
    pub fiat_amount: i64,
    pub payment_method: String,
    pub premium: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_buyer_pubkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_seller_pubkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_invoice: Option<String>,
    pub created_at: Option<i64>,
    pub buyer_pubkey: Option<String>,
    pub seller_pubkey: Option<String>,
}

#[allow(dead_code)]
impl SmallOrder {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Option<Uuid>,
        kind: Option<Kind>,
        status: Option<Status>,
        amount: i64,
        fiat_code: String,
        fiat_amount: i64,
        payment_method: String,
        premium: i64,
        master_buyer_pubkey: Option<String>,
        master_seller_pubkey: Option<String>,
        buyer_invoice: Option<String>,
        created_at: Option<i64>,
        buyer_pubkey: Option<String>,
        seller_pubkey: Option<String>,
    ) -> Self {
        Self {
            id,
            kind,
            status,
            amount,
            fiat_code,
            fiat_amount,
            payment_method,
            premium,
            master_buyer_pubkey,
            master_seller_pubkey,
            buyer_invoice,
            created_at,
            buyer_pubkey,
            seller_pubkey,
        }
    }
    /// New order from json string
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    /// Get order as json string
    pub fn as_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}
