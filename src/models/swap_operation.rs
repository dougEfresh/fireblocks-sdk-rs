// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    crate::models,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwapOperation {
    /// The id of the swap operation
    #[serde(rename = "id")]
    pub id: String,
    /// The id of the vault account or account id
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// The ID of the provider
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "category")]
    pub category: models::ProviderCategoryEnum,
    #[serde(rename = "protocol")]
    pub protocol: models::SwapProviderProtocolsEnum,
    /// **CREATED** – The swap request has been created but not yet started.
    /// **PENDING_USER_ACTION** – Awaiting a user action (e.g. signature or
    /// approval). **PENDING_PROVIDER_ACTION** – Awaiting the provider to
    /// process the request. **PROCESSING** – The swap is actively being
    /// executed on‐chain. **COMPLETED** – The swap has finished successfully.
    /// **CANCELED** – The swap was cancelled by user or provider before
    /// completion. **FAILED** – The swap attempted but encountered an error.
    #[serde(rename = "status")]
    pub status: Status,
    /// The amount of tokens the swapper will provide
    #[serde(rename = "inputAmount")]
    pub input_amount: String,
    /// The id of the asset the swapper will provide
    #[serde(rename = "inputAsset")]
    pub input_asset: String,
    /// The slippage tolerance is a percentage. The slippage tolerance is the
    /// maximum amount the price can change between the time the transaction is
    /// submitted and the time it is executed
    #[serde(rename = "slippageTolerance")]
    pub slippage_tolerance: f64,
    /// The minimum amount of tokens the swapper will receive
    #[serde(rename = "outputMinAmount")]
    pub output_min_amount: String,
    /// Maximum amount of tokens that the swapper will receive
    #[serde(rename = "outputMaxAmount")]
    pub output_max_amount: String,
    /// The id of the asset the swapper will receive
    #[serde(rename = "outputAsset")]
    pub output_asset: String,
    /// Final amount of tokens that the swapper will receive
    #[serde(
        rename = "outputFinalAmount",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub output_final_amount: Option<Option<String>>,
    /// The required actions for the swap, including the type of action, the
    /// status of the action, and the transaction id
    #[serde(rename = "requiredActions")]
    pub required_actions: Vec<models::SwapRequiredAction>,
    #[serde(
        rename = "error",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<Option<models::SwapFlowError>>,
    /// The creation time of the swap operation (ISO Date time).
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The last update time of the swap operation (ISO Date time).
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// Fireblocks user id that issued the swap
    #[serde(rename = "createdBy")]
    pub created_by: uuid::Uuid,
}

impl SwapOperation {
    pub fn new(
        id: String,
        account_id: String,
        provider_id: String,
        category: models::ProviderCategoryEnum,
        protocol: models::SwapProviderProtocolsEnum,
        status: Status,
        input_amount: String,
        input_asset: String,
        slippage_tolerance: f64,
        output_min_amount: String,
        output_max_amount: String,
        output_asset: String,
        required_actions: Vec<models::SwapRequiredAction>,
        created_at: String,
        updated_at: String,
        created_by: uuid::Uuid,
    ) -> SwapOperation {
        SwapOperation {
            id,
            account_id,
            provider_id,
            category,
            protocol,
            status,
            input_amount,
            input_asset,
            slippage_tolerance,
            output_min_amount,
            output_max_amount,
            output_asset,
            output_final_amount: None,
            required_actions,
            error: None,
            created_at,
            updated_at,
            created_by,
        }
    }
}
/// **CREATED** – The swap request has been created but not yet started.
/// **PENDING_USER_ACTION** – Awaiting a user action (e.g. signature or
/// approval). **PENDING_PROVIDER_ACTION** – Awaiting the provider to process
/// the request. **PROCESSING** – The swap is actively being executed on‐chain.
/// **COMPLETED** – The swap has finished successfully. **CANCELED** – The swap
/// was cancelled by user or provider before completion. **FAILED** – The swap
/// attempted but encountered an error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "TRANSACTION_IN_PROGRESS")]
    TransactionInProgress,
    #[serde(rename = "PENDING_PROVIDER_ACTION")]
    PendingProviderAction,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "FAILED")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Created
    }
}
