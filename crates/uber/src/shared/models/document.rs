//! Verification document types for driver onboarding.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The kind of document submitted for verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DocumentType {
    /// Government-issued national identity card.
    NationalId,
    /// Driving licence / driver's permit.
    DrivingLicense,
    /// Vehicle registration certificate.
    VehicleRegistration,
    /// Photo of the vehicle.
    VehiclePhoto,
}

/// The verification outcome for a submitted document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum VerificationStatus {
    /// The document is awaiting review.
    Pending,
    /// The document has been approved.
    Approved,
    /// The document was rejected.
    Rejected {
        /// Human-readable reason for the rejection.
        reason: String,
    },
}

/// A document submitted by a driver for identity or vehicle verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationDocument {
    /// Unique identifier for the document record.
    pub id: String,
    /// ID of the user who submitted the document.
    pub user_id: String,
    /// The type of document submitted.
    pub doc_type: DocumentType,
    /// URL to the uploaded document image.
    pub image_url: String,
    /// Current verification status.
    pub status: VerificationStatus,
    /// Timestamp when the document was submitted.
    pub submitted_at: DateTime<Utc>,
}
