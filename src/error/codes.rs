/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};

use serde::{Deserialize, Serialize};

/// Deribit RPC Error Codes
///
/// Complete enumeration of all error codes returned by the Deribit API
/// as documented in the official API documentation v2.1.1
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum DeribitErrorCode {
    /// Success, No error
    Success,
    /// Authorization issue, invalid or absent signature etc.
    AuthorizationRequired,
    /// Some general failure, no public information available
    Error,
    /// Order quantity is too low
    QtyTooLow,
    /// Rejection, order overlap is found and self-trading is not enabled
    OrderOverlap,
    /// Attempt to operate with order that can't be found by specified id or label
    OrderNotFound,
    /// Price is too low, limit defines current limit for the operation
    PriceTooLow,
    /// Price is too low for current index, limit defines current bottom limit
    PriceTooLow4Idx,
    /// Price is too high, limit defines current up limit for the operation
    PriceTooHigh,
    /// Account has not enough funds for the operation
    NotEnoughFunds,
    /// Attempt of doing something with closed order
    AlreadyClosed,
    /// This price is not allowed for some reason
    PriceNotAllowed,
    /// Operation for an instrument which order book had been closed
    BookClosed,
    /// Total limit of open orders has been exceeded (PME users)
    PmeMaxTotalOpenOrders,
    /// Limit of count of futures' open orders has been exceeded (PME users)
    PmeMaxFutureOpenOrders,
    /// Limit of count of options' open orders has been exceeded (PME users)
    PmeMaxOptionOpenOrders,
    /// Limit of size for futures has been exceeded (PME users)
    PmeMaxFutureOpenOrdersSize,
    /// Limit of size for options has been exceeded (PME users)
    PmeMaxOptionOpenOrdersSize,
    /// Limit of size for futures has been exceeded (non-PME users)
    NonPmeMaxFuturePositionSize,
    /// Trading is temporary locked by the admin
    LockedByAdmin,
    /// Instrument name is not valid
    InvalidOrUnsupportedInstrument,
    /// Amount is not valid
    InvalidAmount,
    /// Quantity was not recognized as a valid number (for API v1)
    InvalidQuantity,
    /// Price was not recognized as a valid number
    InvalidPrice,
    /// max_show parameter was not recognized as a valid number
    InvalidMaxShow,
    /// Order id is missing or its format was not recognized as valid
    InvalidOrderId,
    /// Extra precision of the price is not supported
    PricePrecisionExceeded,
    /// Futures contract amount was not recognized as integer
    NonIntegerContractAmount,
    /// Allowed request rate has been exceeded
    TooManyRequests,
    /// Attempt to operate with not own order
    NotOwnerOfOrder,
    /// REST request where Websocket is expected
    MustBeWebsocketRequest,
    /// Some of the arguments are not recognized as valid
    InvalidArgsForInstrument,
    /// Total cost is too low
    WholeCostTooLow,
    /// Method is not implemented yet
    NotImplemented,
    /// Trigger price is too high
    TriggerPriceTooHigh,
    /// Trigger price is too low
    TriggerPriceTooLow,
    /// Max Show Amount is not valid
    InvalidMaxShowAmount,
    /// Limit of total size for short options positions has been exceeded (non-PME users)
    NonPmeTotalShortOptionsPositionsSize,
    /// Limit of open risk reducing orders has been reached (PME users)
    PmeMaxRiskReducingOrders,
    /// User does not have sufficient spot reserves or negative impact on portfolio margin
    NotEnoughFundsInCurrency,
    /// Request can't be processed right now and should be retried
    Retry,
    /// Settlement is in progress
    SettlementInProgress,
    /// Price has to be rounded to an instrument tick size
    PriceWrongTick,
    /// Trigger Price has to be rounded to an instrument tick size
    TriggerPriceWrongTick,
    /// Liquidation order can't be cancelled
    CanNotCancelLiquidationOrder,
    /// Liquidation order can't be edited
    CanNotEditLiquidationOrder,
    /// Reached limit of pending Matching Engine requests for user
    MatchingEngineQueueFull,
    /// The requested operation is not available on this server
    NotOnThisServer,
    /// Enabling Cancel On Disconnect for the connection failed
    CancelOnDisconnectFailed,
    /// The client has sent too many public requests that have not yet been executed
    TooManyConcurrentRequests,
    /// Spot trading is disabled for users in reduce only mode
    DisabledWhilePositionLock,
    /// This request is not allowed in regards to the filled order
    AlreadyFilled,
    /// Total limit of open orders on spot instruments has been exceeded
    MaxSpotOpenOrders,
    /// Price modification for post only order is not possible
    PostOnlyPriceModificationNotPossible,
    /// Limit of quantity per currency for spot instruments has been exceeded
    MaxSpotOrderQuantity,
    /// Some invalid input has been detected
    InvalidArguments,
    /// Some rejects which are not considered as very often
    OtherReject,
    /// Some errors which are not considered as very often
    OtherError,
    /// Allowed amount of trigger orders has been exceeded
    NoMoreTriggers,
    /// Invalid trigger price in relation to the last trade, index or market price
    InvalidTriggerPrice,
    /// Instrument already not available for trading
    OutdatedInstrumentForIvOrder,
    /// Advanced orders are not available for futures
    NoAdvForFutures,
    /// Advanced post-only orders are not supported yet
    NoAdvPostonly,
    /// Advanced order properties can't be set if the order is not advanced
    NotAdvOrder,
    /// Permission for the operation has been denied
    PermissionDenied,
    /// Bad argument has been passed
    BadArgument,
    /// Attempt to do open order operations with the not open order
    NotOpenOrder,
    /// Event name has not been recognized
    InvalidEvent,
    /// At several minutes to instrument expiration, advanced IV orders are not allowed
    OutdatedInstrument,
    /// The specified combination of arguments is not supported
    UnsupportedArgCombination,
    /// Wrong Max Show for options
    WrongMaxShowForOption,
    /// Several bad arguments have been passed
    BadArguments,
    /// Request has not been parsed properly
    BadRequest,
    /// System is under maintenance
    SystemMaintenance,
    /// Subscription error
    SubscribeErrorUnsubscribed,
    /// Specified transfer is not found
    TransferNotFound,
    /// Request rejected due to reject_post_only flag
    PostOnlyReject,
    /// Post only flag not allowed for given order type
    PostOnlyNotAllowed,
    /// Unauthenticated public requests were temporarily disabled
    UnauthenticatedPublicRequestsTemporarilyDisabled,
    /// Invalid address
    InvalidAddr,
    /// Invalid address for the transfer
    InvalidTransferAddress,
    /// The address already exists
    AddressAlreadyExist,
    /// Limit of allowed addresses has been reached
    MaxAddrCountExceeded,
    /// Some unhandled error on server
    InternalServerError,
    /// Deposit address creation has been disabled by admin
    DisabledDepositAddressCreation,
    /// Withdrawal instead of transfer
    AddressBelongsToUser,
    /// Deposit address not specified
    NoDepositAddress,
    /// Account locked
    AccountLocked,
    /// Limit of subaccounts is reached
    TooManySubaccounts,
    /// The input is not allowed as the name of subaccount
    WrongSubaccountName,
    /// The number of failed login attempts is limited
    LoginOverLimit,
    /// The number of registration requests is limited
    RegistrationOverLimit,
    /// The country is banned (possibly via IP check)
    CountryIsBanned,
    /// Transfer is not allowed
    TransferNotAllowed,
    /// Too many failed security key authorizations
    SecurityKeyAuthorizationOverLimit,
    /// Invalid credentials have been used
    InvalidCredentials,
    /// Password confirmation error
    PwdMatchError,
    /// Invalid Security Code
    SecurityError,
    /// User's security code has been changed or wrong
    UserNotFound,
    /// Request failed because of invalid input or internal failure
    RequestFailed,
    /// Wrong or expired authorization token or bad signature
    Unauthorized,
    /// Invalid input, missing value
    ValueRequired,
    /// Input is too short
    ValueTooShort,
    /// Subaccount restrictions
    UnavailableInSubaccount,
    /// Unsupported or invalid phone number
    InvalidPhoneNumber,
    /// SMS sending failed -- phone number is wrong
    CannotSendSms,
    /// Invalid SMS code
    InvalidSmsCode,
    /// Invalid input
    InvalidInput,
    /// Invalid content type of the request
    InvalidContentType,
    /// Closed, expired order book
    OrderbookClosed,
    /// Instrument is not found, invalid instrument name
    NotFound,
    /// Not enough permissions to execute the request, forbidden
    Forbidden,
    /// API method temporarily switched off by the administrator
    MethodSwitchedOffByAdmin,
    /// The requested service is not responding or processing takes too long
    TemporarilyUnavailable,
    /// Order has been rejected due to the MMP trigger
    MmpTrigger,
    /// API method allowed only for verified users
    VerificationRequired,
    /// Request allowed only for orders uniquely identified by given label
    NonUniqueOrderLabel,
    /// Maximal number of tokens allowed reached
    NoMoreSecurityKeysAllowed,
    /// Limit of active combo books was reached
    ActiveComboLimitReached,
    /// Action is temporarily unavailable for combo books
    UnavailableForComboBooks,
    /// KYC verification data is insufficient for external service provider
    IncompleteKycData,
    /// User is not a MMP user
    MmpRequired,
    /// Cancel-on-Disconnect is not enabled for the connection
    CodNotEnabled,
    /// Quotes are still frozen after previous cancel
    QuotesFrozen,
    /// Error returned after the user tried to edit/delete an API key with insufficient scope
    ScopeExceeded,
    /// Method is currently not available
    Unavailable,
    /// Request was cancelled by the user with other api request
    RequestCancelledByUser,
    /// Edit request was replaced by other one
    Replaced,
    /// Raw subscriptions are not available for unauthorized requests
    RawSubscriptionsNotAvailableForUnauthorized,
    /// The client cannot execute the request yet, should wait
    MovePositionsOverLimit,
    /// The coupon has already been used by current account
    CouponAlreadyUsed,
    /// Sharing of KYC data with a third party provider was already initiated
    KycTransferAlreadyInitiated,
    /// Unknown error code
    Unknown(i32),
}

impl DeribitErrorCode {
    /// Get the numeric error code
    pub fn code(&self) -> i32 {
        self.clone().into()
    }

    /// Get the short error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::AuthorizationRequired => "authorization_required",
            Self::Error => "error",
            Self::QtyTooLow => "qty_too_low",
            Self::OrderOverlap => "order_overlap",
            Self::OrderNotFound => "order_not_found",
            Self::PriceTooLow => "price_too_low",
            Self::PriceTooLow4Idx => "price_too_low4idx",
            Self::PriceTooHigh => "price_too_high",
            Self::NotEnoughFunds => "not_enough_funds",
            Self::AlreadyClosed => "already_closed",
            Self::PriceNotAllowed => "price_not_allowed",
            Self::BookClosed => "book_closed",
            Self::PmeMaxTotalOpenOrders => "pme_max_total_open_orders",
            Self::PmeMaxFutureOpenOrders => "pme_max_future_open_orders",
            Self::PmeMaxOptionOpenOrders => "pme_max_option_open_orders",
            Self::PmeMaxFutureOpenOrdersSize => "pme_max_future_open_orders_size",
            Self::PmeMaxOptionOpenOrdersSize => "pme_max_option_open_orders_size",
            Self::NonPmeMaxFuturePositionSize => "non_pme_max_future_position_size",
            Self::LockedByAdmin => "locked_by_admin",
            Self::InvalidOrUnsupportedInstrument => "invalid_or_unsupported_instrument",
            Self::InvalidAmount => "invalid_amount",
            Self::InvalidQuantity => "invalid_quantity",
            Self::InvalidPrice => "invalid_price",
            Self::InvalidMaxShow => "invalid_max_show",
            Self::InvalidOrderId => "invalid_order_id",
            Self::PricePrecisionExceeded => "price_precision_exceeded",
            Self::NonIntegerContractAmount => "non_integer_contract_amount",
            Self::TooManyRequests => "too_many_requests",
            Self::NotOwnerOfOrder => "not_owner_of_order",
            Self::MustBeWebsocketRequest => "must_be_websocket_request",
            Self::InvalidArgsForInstrument => "invalid_args_for_instrument",
            Self::WholeCostTooLow => "whole_cost_too_low",
            Self::NotImplemented => "not_implemented",
            Self::TriggerPriceTooHigh => "trigger_price_too_high",
            Self::TriggerPriceTooLow => "trigger_price_too_low",
            Self::InvalidMaxShowAmount => "invalid_max_show_amount",
            Self::NonPmeTotalShortOptionsPositionsSize => {
                "non_pme_total_short_options_positions_size"
            }
            Self::PmeMaxRiskReducingOrders => "pme_max_risk_reducing_orders",
            Self::NotEnoughFundsInCurrency => "not_enough_funds_in_currency",
            Self::Retry => "retry",
            Self::SettlementInProgress => "settlement_in_progress",
            Self::PriceWrongTick => "price_wrong_tick",
            Self::TriggerPriceWrongTick => "trigger_price_wrong_tick",
            Self::CanNotCancelLiquidationOrder => "can_not_cancel_liquidation_order",
            Self::CanNotEditLiquidationOrder => "can_not_edit_liquidation_order",
            Self::MatchingEngineQueueFull => "matching_engine_queue_full",
            Self::NotOnThisServer => "not_on_this_server",
            Self::CancelOnDisconnectFailed => "cancel_on_disconnect_failed",
            Self::TooManyConcurrentRequests => "too_many_concurrent_requests",
            Self::DisabledWhilePositionLock => "disabled_while_position_lock",
            Self::AlreadyFilled => "already_filled",
            Self::MaxSpotOpenOrders => "max_spot_open_orders",
            Self::PostOnlyPriceModificationNotPossible => {
                "post_only_price_modification_not_possible"
            }
            Self::MaxSpotOrderQuantity => "max_spot_order_quantity",
            Self::InvalidArguments => "invalid_arguments",
            Self::OtherReject => "other_reject",
            Self::OtherError => "other_error",
            Self::NoMoreTriggers => "no_more_triggers",
            Self::InvalidTriggerPrice => "invalid_trigger_price",
            Self::OutdatedInstrumentForIvOrder => "outdated_instrument_for_IV_order",
            Self::NoAdvForFutures => "no_adv_for_futures",
            Self::NoAdvPostonly => "no_adv_postonly",
            Self::NotAdvOrder => "not_adv_order",
            Self::PermissionDenied => "permission_denied",
            Self::BadArgument => "bad_argument",
            Self::NotOpenOrder => "not_open_order",
            Self::InvalidEvent => "invalid_event",
            Self::OutdatedInstrument => "outdated_instrument",
            Self::UnsupportedArgCombination => "unsupported_arg_combination",
            Self::WrongMaxShowForOption => "wrong_max_show_for_option",
            Self::BadArguments => "bad_arguments",
            Self::BadRequest => "bad_request",
            Self::SystemMaintenance => "system_maintenance",
            Self::SubscribeErrorUnsubscribed => "subscribe_error_unsubscribed",
            Self::TransferNotFound => "transfer_not_found",
            Self::PostOnlyReject => "post_only_reject",
            Self::PostOnlyNotAllowed => "post_only_not_allowed",
            Self::UnauthenticatedPublicRequestsTemporarilyDisabled => {
                "unauthenticated_public_requests_temporarily_disabled"
            }
            Self::InvalidAddr => "invalid_addr",
            Self::InvalidTransferAddress => "invalid_transfer_address",
            Self::AddressAlreadyExist => "address_already_exist",
            Self::MaxAddrCountExceeded => "max_addr_count_exceeded",
            Self::InternalServerError => "internal_server_error",
            Self::DisabledDepositAddressCreation => "disabled_deposit_address_creation",
            Self::AddressBelongsToUser => "address_belongs_to_user",
            Self::NoDepositAddress => "no_deposit_address",
            Self::AccountLocked => "account_locked",
            Self::TooManySubaccounts => "too_many_subaccounts",
            Self::WrongSubaccountName => "wrong_subaccount_name",
            Self::LoginOverLimit => "login_over_limit",
            Self::RegistrationOverLimit => "registration_over_limit",
            Self::CountryIsBanned => "country_is_banned",
            Self::TransferNotAllowed => "transfer_not_allowed",
            Self::SecurityKeyAuthorizationOverLimit => "security_key_authorization_over_limit",
            Self::InvalidCredentials => "invalid_credentials",
            Self::PwdMatchError => "pwd_match_error",
            Self::SecurityError => "security_error",
            Self::UserNotFound => "user_not_found",
            Self::RequestFailed => "request_failed",
            Self::Unauthorized => "unauthorized",
            Self::ValueRequired => "value_required",
            Self::ValueTooShort => "value_too_short",
            Self::UnavailableInSubaccount => "unavailable_in_subaccount",
            Self::InvalidPhoneNumber => "invalid_phone_number",
            Self::CannotSendSms => "cannot_send_sms",
            Self::InvalidSmsCode => "invalid_sms_code",
            Self::InvalidInput => "invalid_input",
            Self::InvalidContentType => "invalid_content_type",
            Self::OrderbookClosed => "orderbook_closed",
            Self::NotFound => "not_found",
            Self::Forbidden => "forbidden",
            Self::MethodSwitchedOffByAdmin => "method_switched_off_by_admin",
            Self::TemporarilyUnavailable => "temporarily_unavailable",
            Self::MmpTrigger => "mmp_trigger",
            Self::VerificationRequired => "verification_required",
            Self::NonUniqueOrderLabel => "non_unique_order_label",
            Self::NoMoreSecurityKeysAllowed => "no_more_security_keys_allowed",
            Self::ActiveComboLimitReached => "active_combo_limit_reached",
            Self::UnavailableForComboBooks => "unavailable_for_combo_books",
            Self::IncompleteKycData => "incomplete_KYC_data",
            Self::MmpRequired => "mmp_required",
            Self::CodNotEnabled => "cod_not_enabled",
            Self::QuotesFrozen => "quotes_frozen",
            Self::ScopeExceeded => "scope_exceeded",
            Self::Unavailable => "unavailable",
            Self::RequestCancelledByUser => "request_cancelled_by_user",
            Self::Replaced => "replaced",
            Self::RawSubscriptionsNotAvailableForUnauthorized => {
                "raw_subscriptions_not_available_for_unauthorized"
            }
            Self::MovePositionsOverLimit => "move_positions_over_limit",
            Self::CouponAlreadyUsed => "coupon_already_used",
            Self::KycTransferAlreadyInitiated => "KYC_transfer_already_initiated",
            Self::Unknown(_) => "unknown_error",
        }
    }

    /// Check if this is a success code
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    /// Check if this is an authorization error
    pub fn is_authorization_error(&self) -> bool {
        matches!(self, Self::AuthorizationRequired | Self::Unauthorized)
    }

    /// Check if this is a rate limiting error
    pub fn is_rate_limit_error(&self) -> bool {
        matches!(
            self,
            Self::TooManyRequests | Self::TooManyConcurrentRequests
        )
    }

    /// Check if this is a validation error
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidAmount
                | Self::InvalidPrice
                | Self::InvalidQuantity
                | Self::InvalidOrderId
                | Self::InvalidArguments
                | Self::BadArgument
                | Self::BadArguments
                | Self::InvalidInput
        )
    }

    /// Check if this is a trading error
    pub fn is_trading_error(&self) -> bool {
        matches!(
            self,
            Self::QtyTooLow
                | Self::OrderOverlap
                | Self::OrderNotFound
                | Self::PriceTooLow
                | Self::PriceTooHigh
                | Self::NotEnoughFunds
                | Self::AlreadyClosed
                | Self::PriceNotAllowed
                | Self::BookClosed
        )
    }
}

// Error trait implementation
impl std::error::Error for DeribitErrorCode {}

// Conversion from DeribitErrorCode to i32 (required for serde)
impl From<DeribitErrorCode> for i32 {
    fn from(error: DeribitErrorCode) -> Self {
        match error {
            DeribitErrorCode::Success => 0,
            DeribitErrorCode::AuthorizationRequired => 10000,
            DeribitErrorCode::Error => 10001,
            DeribitErrorCode::QtyTooLow => 10002,
            DeribitErrorCode::OrderOverlap => 10003,
            DeribitErrorCode::OrderNotFound => 10004,
            DeribitErrorCode::PriceTooLow => 10005,
            DeribitErrorCode::PriceTooLow4Idx => 10006,
            DeribitErrorCode::PriceTooHigh => 10007,
            DeribitErrorCode::NotEnoughFunds => 10009,
            DeribitErrorCode::AlreadyClosed => 10010,
            DeribitErrorCode::PriceNotAllowed => 10011,
            DeribitErrorCode::BookClosed => 10012,
            DeribitErrorCode::PmeMaxTotalOpenOrders => 10013,
            DeribitErrorCode::PmeMaxFutureOpenOrders => 10014,
            DeribitErrorCode::PmeMaxOptionOpenOrders => 10015,
            DeribitErrorCode::PmeMaxFutureOpenOrdersSize => 10016,
            DeribitErrorCode::PmeMaxOptionOpenOrdersSize => 10017,
            DeribitErrorCode::NonPmeMaxFuturePositionSize => 10018,
            DeribitErrorCode::LockedByAdmin => 10019,
            DeribitErrorCode::InvalidOrUnsupportedInstrument => 10020,
            DeribitErrorCode::InvalidAmount => 10021,
            DeribitErrorCode::InvalidQuantity => 10022,
            DeribitErrorCode::InvalidPrice => 10023,
            DeribitErrorCode::InvalidMaxShow => 10024,
            DeribitErrorCode::InvalidOrderId => 10025,
            DeribitErrorCode::PricePrecisionExceeded => 10026,
            DeribitErrorCode::NonIntegerContractAmount => 10027,
            DeribitErrorCode::TooManyRequests => 10028,
            DeribitErrorCode::NotOwnerOfOrder => 10029,
            DeribitErrorCode::MustBeWebsocketRequest => 10030,
            DeribitErrorCode::InvalidArgsForInstrument => 10031,
            DeribitErrorCode::WholeCostTooLow => 10032,
            DeribitErrorCode::NotImplemented => 10033,
            DeribitErrorCode::TriggerPriceTooHigh => 10034,
            DeribitErrorCode::TriggerPriceTooLow => 10035,
            DeribitErrorCode::InvalidMaxShowAmount => 10036,
            DeribitErrorCode::NonPmeTotalShortOptionsPositionsSize => 10037,
            DeribitErrorCode::PmeMaxRiskReducingOrders => 10038,
            DeribitErrorCode::NotEnoughFundsInCurrency => 10039,
            DeribitErrorCode::Retry => 10040,
            DeribitErrorCode::SettlementInProgress => 10041,
            DeribitErrorCode::PriceWrongTick => 10043,
            DeribitErrorCode::TriggerPriceWrongTick => 10044,
            DeribitErrorCode::CanNotCancelLiquidationOrder => 10045,
            DeribitErrorCode::CanNotEditLiquidationOrder => 10046,
            DeribitErrorCode::MatchingEngineQueueFull => 10047,
            DeribitErrorCode::NotOnThisServer => 10048,
            DeribitErrorCode::CancelOnDisconnectFailed => 10049,
            DeribitErrorCode::TooManyConcurrentRequests => 10066,
            DeribitErrorCode::DisabledWhilePositionLock => 10072,
            DeribitErrorCode::AlreadyFilled => 11008,
            DeribitErrorCode::MaxSpotOpenOrders => 11013,
            DeribitErrorCode::PostOnlyPriceModificationNotPossible => 11021,
            DeribitErrorCode::MaxSpotOrderQuantity => 11022,
            DeribitErrorCode::InvalidArguments => 11029,
            DeribitErrorCode::OtherReject => 11030,
            DeribitErrorCode::OtherError => 11031,
            DeribitErrorCode::NoMoreTriggers => 11035,
            DeribitErrorCode::InvalidTriggerPrice => 11036,
            DeribitErrorCode::OutdatedInstrumentForIvOrder => 11037,
            DeribitErrorCode::NoAdvForFutures => 11038,
            DeribitErrorCode::NoAdvPostonly => 11039,
            DeribitErrorCode::NotAdvOrder => 11041,
            DeribitErrorCode::PermissionDenied => 11042,
            DeribitErrorCode::BadArgument => 11043,
            DeribitErrorCode::NotOpenOrder => 11044,
            DeribitErrorCode::InvalidEvent => 11045,
            DeribitErrorCode::OutdatedInstrument => 11046,
            DeribitErrorCode::UnsupportedArgCombination => 11047,
            DeribitErrorCode::WrongMaxShowForOption => 11048,
            DeribitErrorCode::BadArguments => 11049,
            DeribitErrorCode::BadRequest => 11050,
            DeribitErrorCode::SystemMaintenance => 11051,
            DeribitErrorCode::SubscribeErrorUnsubscribed => 11052,
            DeribitErrorCode::TransferNotFound => 11053,
            DeribitErrorCode::PostOnlyReject => 11054,
            DeribitErrorCode::PostOnlyNotAllowed => 11055,
            DeribitErrorCode::UnauthenticatedPublicRequestsTemporarilyDisabled => 11056,
            DeribitErrorCode::InvalidAddr => 11090,
            DeribitErrorCode::InvalidTransferAddress => 11091,
            DeribitErrorCode::AddressAlreadyExist => 11092,
            DeribitErrorCode::MaxAddrCountExceeded => 11093,
            DeribitErrorCode::InternalServerError => 11094,
            DeribitErrorCode::DisabledDepositAddressCreation => 11095,
            DeribitErrorCode::AddressBelongsToUser => 11096,
            DeribitErrorCode::NoDepositAddress => 11097,
            DeribitErrorCode::AccountLocked => 11098,
            DeribitErrorCode::TooManySubaccounts => 12001,
            DeribitErrorCode::WrongSubaccountName => 12002,
            DeribitErrorCode::LoginOverLimit => 12003,
            DeribitErrorCode::RegistrationOverLimit => 12004,
            DeribitErrorCode::CountryIsBanned => 12005,
            DeribitErrorCode::TransferNotAllowed => 12100,
            DeribitErrorCode::SecurityKeyAuthorizationOverLimit => 12998,
            DeribitErrorCode::InvalidCredentials => 13004,
            DeribitErrorCode::PwdMatchError => 13005,
            DeribitErrorCode::SecurityError => 13006,
            DeribitErrorCode::UserNotFound => 13007,
            DeribitErrorCode::RequestFailed => 13008,
            DeribitErrorCode::Unauthorized => 13009,
            DeribitErrorCode::ValueRequired => 13010,
            DeribitErrorCode::ValueTooShort => 13011,
            DeribitErrorCode::UnavailableInSubaccount => 13012,
            DeribitErrorCode::InvalidPhoneNumber => 13013,
            DeribitErrorCode::CannotSendSms => 13014,
            DeribitErrorCode::InvalidSmsCode => 13015,
            DeribitErrorCode::InvalidInput => 13016,
            DeribitErrorCode::InvalidContentType => 13018,
            DeribitErrorCode::OrderbookClosed => 13019,
            DeribitErrorCode::NotFound => 13020,
            DeribitErrorCode::Forbidden => 13021,
            DeribitErrorCode::MethodSwitchedOffByAdmin => 13025,
            DeribitErrorCode::TemporarilyUnavailable => 13028,
            DeribitErrorCode::MmpTrigger => 13030,
            DeribitErrorCode::VerificationRequired => 13031,
            DeribitErrorCode::NonUniqueOrderLabel => 13032,
            DeribitErrorCode::NoMoreSecurityKeysAllowed => 13034,
            DeribitErrorCode::ActiveComboLimitReached => 13035,
            DeribitErrorCode::UnavailableForComboBooks => 13036,
            DeribitErrorCode::IncompleteKycData => 13037,
            DeribitErrorCode::MmpRequired => 13040,
            DeribitErrorCode::CodNotEnabled => 13042,
            DeribitErrorCode::QuotesFrozen => 13043,
            DeribitErrorCode::ScopeExceeded => 13403,
            DeribitErrorCode::Unavailable => 13503,
            DeribitErrorCode::RequestCancelledByUser => 13666,
            DeribitErrorCode::Replaced => 13777,
            DeribitErrorCode::RawSubscriptionsNotAvailableForUnauthorized => 13778,
            DeribitErrorCode::MovePositionsOverLimit => 13780,
            DeribitErrorCode::CouponAlreadyUsed => 13781,
            DeribitErrorCode::KycTransferAlreadyInitiated => 13791,
            DeribitErrorCode::Unknown(code) => code,
        }
    }
}

// Conversion from i32 to DeribitErrorCode (required for serde)
impl From<i32> for DeribitErrorCode {
    fn from(code: i32) -> Self {
        match code {
            0 => DeribitErrorCode::Success,
            10000 => DeribitErrorCode::AuthorizationRequired,
            10001 => DeribitErrorCode::Error,
            10002 => DeribitErrorCode::QtyTooLow,
            10003 => DeribitErrorCode::OrderOverlap,
            10004 => DeribitErrorCode::OrderNotFound,
            10005 => DeribitErrorCode::PriceTooLow,
            10006 => DeribitErrorCode::PriceTooLow4Idx,
            10007 => DeribitErrorCode::PriceTooHigh,
            10009 => DeribitErrorCode::NotEnoughFunds,
            10010 => DeribitErrorCode::AlreadyClosed,
            10011 => DeribitErrorCode::PriceNotAllowed,
            10012 => DeribitErrorCode::BookClosed,
            10013 => DeribitErrorCode::PmeMaxTotalOpenOrders,
            10014 => DeribitErrorCode::PmeMaxFutureOpenOrders,
            10015 => DeribitErrorCode::PmeMaxOptionOpenOrders,
            10016 => DeribitErrorCode::PmeMaxFutureOpenOrdersSize,
            10017 => DeribitErrorCode::PmeMaxOptionOpenOrdersSize,
            10018 => DeribitErrorCode::NonPmeMaxFuturePositionSize,
            10019 => DeribitErrorCode::LockedByAdmin,
            10020 => DeribitErrorCode::InvalidOrUnsupportedInstrument,
            10021 => DeribitErrorCode::InvalidAmount,
            10022 => DeribitErrorCode::InvalidQuantity,
            10023 => DeribitErrorCode::InvalidPrice,
            10024 => DeribitErrorCode::InvalidMaxShow,
            10025 => DeribitErrorCode::InvalidOrderId,
            10026 => DeribitErrorCode::PricePrecisionExceeded,
            10027 => DeribitErrorCode::NonIntegerContractAmount,
            10028 => DeribitErrorCode::TooManyRequests,
            10029 => DeribitErrorCode::NotOwnerOfOrder,
            10030 => DeribitErrorCode::MustBeWebsocketRequest,
            10031 => DeribitErrorCode::InvalidArgsForInstrument,
            10032 => DeribitErrorCode::WholeCostTooLow,
            10033 => DeribitErrorCode::NotImplemented,
            10034 => DeribitErrorCode::TriggerPriceTooHigh,
            10035 => DeribitErrorCode::TriggerPriceTooLow,
            10036 => DeribitErrorCode::InvalidMaxShowAmount,
            10037 => DeribitErrorCode::NonPmeTotalShortOptionsPositionsSize,
            10038 => DeribitErrorCode::PmeMaxRiskReducingOrders,
            10039 => DeribitErrorCode::NotEnoughFundsInCurrency,
            10040 => DeribitErrorCode::Retry,
            10041 => DeribitErrorCode::SettlementInProgress,
            10043 => DeribitErrorCode::PriceWrongTick,
            10044 => DeribitErrorCode::TriggerPriceWrongTick,
            10045 => DeribitErrorCode::CanNotCancelLiquidationOrder,
            10046 => DeribitErrorCode::CanNotEditLiquidationOrder,
            10047 => DeribitErrorCode::MatchingEngineQueueFull,
            10048 => DeribitErrorCode::NotOnThisServer,
            10049 => DeribitErrorCode::CancelOnDisconnectFailed,
            10066 => DeribitErrorCode::TooManyConcurrentRequests,
            10072 => DeribitErrorCode::DisabledWhilePositionLock,
            11008 => DeribitErrorCode::AlreadyFilled,
            11013 => DeribitErrorCode::MaxSpotOpenOrders,
            11021 => DeribitErrorCode::PostOnlyPriceModificationNotPossible,
            11022 => DeribitErrorCode::MaxSpotOrderQuantity,
            11029 => DeribitErrorCode::InvalidArguments,
            11030 => DeribitErrorCode::OtherReject,
            11031 => DeribitErrorCode::OtherError,
            11035 => DeribitErrorCode::NoMoreTriggers,
            11036 => DeribitErrorCode::InvalidTriggerPrice,
            11037 => DeribitErrorCode::OutdatedInstrumentForIvOrder,
            11038 => DeribitErrorCode::NoAdvForFutures,
            11039 => DeribitErrorCode::NoAdvPostonly,
            11041 => DeribitErrorCode::NotAdvOrder,
            11042 => DeribitErrorCode::PermissionDenied,
            11043 => DeribitErrorCode::BadArgument,
            11044 => DeribitErrorCode::NotOpenOrder,
            11045 => DeribitErrorCode::InvalidEvent,
            11046 => DeribitErrorCode::OutdatedInstrument,
            11047 => DeribitErrorCode::UnsupportedArgCombination,
            11048 => DeribitErrorCode::WrongMaxShowForOption,
            11049 => DeribitErrorCode::BadArguments,
            11050 => DeribitErrorCode::BadRequest,
            11051 => DeribitErrorCode::SystemMaintenance,
            11052 => DeribitErrorCode::SubscribeErrorUnsubscribed,
            11053 => DeribitErrorCode::TransferNotFound,
            11054 => DeribitErrorCode::PostOnlyReject,
            11055 => DeribitErrorCode::PostOnlyNotAllowed,
            11056 => DeribitErrorCode::UnauthenticatedPublicRequestsTemporarilyDisabled,
            11090 => DeribitErrorCode::InvalidAddr,
            11091 => DeribitErrorCode::InvalidTransferAddress,
            11092 => DeribitErrorCode::AddressAlreadyExist,
            11093 => DeribitErrorCode::MaxAddrCountExceeded,
            11094 => DeribitErrorCode::InternalServerError,
            11095 => DeribitErrorCode::DisabledDepositAddressCreation,
            11096 => DeribitErrorCode::AddressBelongsToUser,
            11097 => DeribitErrorCode::NoDepositAddress,
            11098 => DeribitErrorCode::AccountLocked,
            12001 => DeribitErrorCode::TooManySubaccounts,
            12002 => DeribitErrorCode::WrongSubaccountName,
            12003 => DeribitErrorCode::LoginOverLimit,
            12004 => DeribitErrorCode::RegistrationOverLimit,
            12005 => DeribitErrorCode::CountryIsBanned,
            12100 => DeribitErrorCode::TransferNotAllowed,
            12998 => DeribitErrorCode::SecurityKeyAuthorizationOverLimit,
            13004 => DeribitErrorCode::InvalidCredentials,
            13005 => DeribitErrorCode::PwdMatchError,
            13006 => DeribitErrorCode::SecurityError,
            13007 => DeribitErrorCode::UserNotFound,
            13008 => DeribitErrorCode::RequestFailed,
            13009 => DeribitErrorCode::Unauthorized,
            13010 => DeribitErrorCode::ValueRequired,
            13011 => DeribitErrorCode::ValueTooShort,
            13012 => DeribitErrorCode::UnavailableInSubaccount,
            13013 => DeribitErrorCode::InvalidPhoneNumber,
            13014 => DeribitErrorCode::CannotSendSms,
            13015 => DeribitErrorCode::InvalidSmsCode,
            13016 => DeribitErrorCode::InvalidInput,
            13018 => DeribitErrorCode::InvalidContentType,
            13019 => DeribitErrorCode::OrderbookClosed,
            13020 => DeribitErrorCode::NotFound,
            13021 => DeribitErrorCode::Forbidden,
            13025 => DeribitErrorCode::MethodSwitchedOffByAdmin,
            13028 => DeribitErrorCode::TemporarilyUnavailable,
            13030 => DeribitErrorCode::MmpTrigger,
            13031 => DeribitErrorCode::VerificationRequired,
            13032 => DeribitErrorCode::NonUniqueOrderLabel,
            13034 => DeribitErrorCode::NoMoreSecurityKeysAllowed,
            13035 => DeribitErrorCode::ActiveComboLimitReached,
            13036 => DeribitErrorCode::UnavailableForComboBooks,
            13037 => DeribitErrorCode::IncompleteKycData,
            13040 => DeribitErrorCode::MmpRequired,
            13042 => DeribitErrorCode::CodNotEnabled,
            13043 => DeribitErrorCode::QuotesFrozen,
            13403 => DeribitErrorCode::ScopeExceeded,
            13503 => DeribitErrorCode::Unavailable,
            13666 => DeribitErrorCode::RequestCancelledByUser,
            13777 => DeribitErrorCode::Replaced,
            13778 => DeribitErrorCode::RawSubscriptionsNotAvailableForUnauthorized,
            13780 => DeribitErrorCode::MovePositionsOverLimit,
            13781 => DeribitErrorCode::CouponAlreadyUsed,
            13791 => DeribitErrorCode::KycTransferAlreadyInitiated,
            _ => DeribitErrorCode::Unknown(code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_conversion() {
        let error = DeribitErrorCode::from(10000);
        assert_eq!(error, DeribitErrorCode::AuthorizationRequired);
        assert_eq!(error.code(), 10000);
        assert_eq!(error.message(), "authorization_required");
    }

    #[test]
    fn test_unknown_error_code() {
        let error = DeribitErrorCode::from(99999);
        assert_eq!(error, DeribitErrorCode::Unknown(99999));
        assert_eq!(error.code(), 99999);
        assert_eq!(error.message(), "unknown_error");
    }

    #[test]
    fn test_success_code() {
        let error = DeribitErrorCode::from(0);
        assert_eq!(error, DeribitErrorCode::Success);
        assert!(error.is_success());
        assert!(!error.is_authorization_error());
    }

    #[test]
    fn test_error_categorization() {
        let auth_error = DeribitErrorCode::AuthorizationRequired;
        assert!(auth_error.is_authorization_error());
        assert!(!auth_error.is_trading_error());

        let trading_error = DeribitErrorCode::QtyTooLow;
        assert!(trading_error.is_trading_error());
        assert!(!trading_error.is_authorization_error());

        let rate_limit_error = DeribitErrorCode::TooManyRequests;
        assert!(rate_limit_error.is_rate_limit_error());
        assert!(!rate_limit_error.is_validation_error());

        let validation_error = DeribitErrorCode::InvalidAmount;
        assert!(validation_error.is_validation_error());
        assert!(!validation_error.is_rate_limit_error());
    }

    #[test]
    fn test_serde_serialization() {
        let error = DeribitErrorCode::AuthorizationRequired;
        let json = serde_json::to_string(&error).unwrap();
        assert_eq!(json, "10000");

        let deserialized: DeribitErrorCode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, error);
    }

    #[test]
    fn test_specific_error_codes() {
        // Test some key error codes to ensure they map correctly
        assert_eq!(DeribitErrorCode::from(0), DeribitErrorCode::Success);
        assert_eq!(DeribitErrorCode::from(10001), DeribitErrorCode::Error);
        assert_eq!(DeribitErrorCode::from(10002), DeribitErrorCode::QtyTooLow);
        assert_eq!(
            DeribitErrorCode::from(13009),
            DeribitErrorCode::Unauthorized
        );
        assert_eq!(DeribitErrorCode::from(13020), DeribitErrorCode::NotFound);

        // Test reverse conversion
        assert_eq!(DeribitErrorCode::Success.code(), 0);
        assert_eq!(DeribitErrorCode::AuthorizationRequired.code(), 10000);
        assert_eq!(DeribitErrorCode::TooManyRequests.code(), 10028);
    }

    #[test]
    fn test_all_error_code_conversions() {
        // Test all error codes for bidirectional conversion
        let test_cases = vec![
            (0, DeribitErrorCode::Success),
            (10000, DeribitErrorCode::AuthorizationRequired),
            (10001, DeribitErrorCode::Error),
            (10002, DeribitErrorCode::QtyTooLow),
            (10003, DeribitErrorCode::OrderOverlap),
            (10004, DeribitErrorCode::OrderNotFound),
            (10005, DeribitErrorCode::PriceTooLow),
            (10006, DeribitErrorCode::PriceTooLow4Idx),
            (10007, DeribitErrorCode::PriceTooHigh),
            (10009, DeribitErrorCode::NotEnoughFunds),
            (10010, DeribitErrorCode::AlreadyClosed),
            (10011, DeribitErrorCode::PriceNotAllowed),
            (10012, DeribitErrorCode::BookClosed),
            (10013, DeribitErrorCode::PmeMaxTotalOpenOrders),
            (10014, DeribitErrorCode::PmeMaxFutureOpenOrders),
            (10015, DeribitErrorCode::PmeMaxOptionOpenOrders),
            (10016, DeribitErrorCode::PmeMaxFutureOpenOrdersSize),
            (10017, DeribitErrorCode::PmeMaxOptionOpenOrdersSize),
            (10018, DeribitErrorCode::NonPmeMaxFuturePositionSize),
            (10019, DeribitErrorCode::LockedByAdmin),
            (10020, DeribitErrorCode::InvalidOrUnsupportedInstrument),
            (10021, DeribitErrorCode::InvalidAmount),
            (10022, DeribitErrorCode::InvalidQuantity),
            (10023, DeribitErrorCode::InvalidPrice),
            (10024, DeribitErrorCode::InvalidMaxShow),
            (10025, DeribitErrorCode::InvalidOrderId),
            (10026, DeribitErrorCode::PricePrecisionExceeded),
            (10027, DeribitErrorCode::NonIntegerContractAmount),
            (10028, DeribitErrorCode::TooManyRequests),
            (10029, DeribitErrorCode::NotOwnerOfOrder),
            (10030, DeribitErrorCode::MustBeWebsocketRequest),
            (10031, DeribitErrorCode::InvalidArgsForInstrument),
            (10032, DeribitErrorCode::WholeCostTooLow),
            (10033, DeribitErrorCode::NotImplemented),
            (10034, DeribitErrorCode::TriggerPriceTooHigh),
            (10035, DeribitErrorCode::TriggerPriceTooLow),
            (10036, DeribitErrorCode::InvalidMaxShowAmount),
            (
                10037,
                DeribitErrorCode::NonPmeTotalShortOptionsPositionsSize,
            ),
            (10038, DeribitErrorCode::PmeMaxRiskReducingOrders),
            (10039, DeribitErrorCode::NotEnoughFundsInCurrency),
            (10040, DeribitErrorCode::Retry),
            (10041, DeribitErrorCode::SettlementInProgress),
            (10043, DeribitErrorCode::PriceWrongTick),
            (10044, DeribitErrorCode::TriggerPriceWrongTick),
            (10045, DeribitErrorCode::CanNotCancelLiquidationOrder),
            (10046, DeribitErrorCode::CanNotEditLiquidationOrder),
            (10047, DeribitErrorCode::MatchingEngineQueueFull),
            (10048, DeribitErrorCode::NotOnThisServer),
            (10049, DeribitErrorCode::CancelOnDisconnectFailed),
            (10066, DeribitErrorCode::TooManyConcurrentRequests),
            (10072, DeribitErrorCode::DisabledWhilePositionLock),
            (11008, DeribitErrorCode::AlreadyFilled),
            (11013, DeribitErrorCode::MaxSpotOpenOrders),
            (
                11021,
                DeribitErrorCode::PostOnlyPriceModificationNotPossible,
            ),
            (11022, DeribitErrorCode::MaxSpotOrderQuantity),
            (11029, DeribitErrorCode::InvalidArguments),
            (11030, DeribitErrorCode::OtherReject),
            (11031, DeribitErrorCode::OtherError),
            (11035, DeribitErrorCode::NoMoreTriggers),
            (11036, DeribitErrorCode::InvalidTriggerPrice),
            (11037, DeribitErrorCode::OutdatedInstrumentForIvOrder),
            (11038, DeribitErrorCode::NoAdvForFutures),
            (11039, DeribitErrorCode::NoAdvPostonly),
            (11041, DeribitErrorCode::NotAdvOrder),
            (11042, DeribitErrorCode::PermissionDenied),
            (11043, DeribitErrorCode::BadArgument),
            (11044, DeribitErrorCode::NotOpenOrder),
            (11045, DeribitErrorCode::InvalidEvent),
            (11046, DeribitErrorCode::OutdatedInstrument),
            (11047, DeribitErrorCode::UnsupportedArgCombination),
            (11048, DeribitErrorCode::WrongMaxShowForOption),
            (11049, DeribitErrorCode::BadArguments),
            (11050, DeribitErrorCode::BadRequest),
            (11051, DeribitErrorCode::SystemMaintenance),
            (11052, DeribitErrorCode::SubscribeErrorUnsubscribed),
            (11053, DeribitErrorCode::TransferNotFound),
            (11054, DeribitErrorCode::PostOnlyReject),
            (11055, DeribitErrorCode::PostOnlyNotAllowed),
            (
                11056,
                DeribitErrorCode::UnauthenticatedPublicRequestsTemporarilyDisabled,
            ),
            (11090, DeribitErrorCode::InvalidAddr),
            (11091, DeribitErrorCode::InvalidTransferAddress),
            (11092, DeribitErrorCode::AddressAlreadyExist),
            (11093, DeribitErrorCode::MaxAddrCountExceeded),
            (11094, DeribitErrorCode::InternalServerError),
            (11095, DeribitErrorCode::DisabledDepositAddressCreation),
            (11096, DeribitErrorCode::AddressBelongsToUser),
            (11097, DeribitErrorCode::NoDepositAddress),
            (11098, DeribitErrorCode::AccountLocked),
            (12001, DeribitErrorCode::TooManySubaccounts),
            (12002, DeribitErrorCode::WrongSubaccountName),
            (12003, DeribitErrorCode::LoginOverLimit),
            (12004, DeribitErrorCode::RegistrationOverLimit),
            (12005, DeribitErrorCode::CountryIsBanned),
            (12100, DeribitErrorCode::TransferNotAllowed),
            (12998, DeribitErrorCode::SecurityKeyAuthorizationOverLimit),
            (13004, DeribitErrorCode::InvalidCredentials),
            (13005, DeribitErrorCode::PwdMatchError),
            (13006, DeribitErrorCode::SecurityError),
            (13007, DeribitErrorCode::UserNotFound),
            (13008, DeribitErrorCode::RequestFailed),
            (13009, DeribitErrorCode::Unauthorized),
            (13010, DeribitErrorCode::ValueRequired),
            (13011, DeribitErrorCode::ValueTooShort),
            (13012, DeribitErrorCode::UnavailableInSubaccount),
            (13013, DeribitErrorCode::InvalidPhoneNumber),
            (13014, DeribitErrorCode::CannotSendSms),
            (13015, DeribitErrorCode::InvalidSmsCode),
            (13016, DeribitErrorCode::InvalidInput),
            (13018, DeribitErrorCode::InvalidContentType),
            (13019, DeribitErrorCode::OrderbookClosed),
            (13020, DeribitErrorCode::NotFound),
            (13021, DeribitErrorCode::Forbidden),
            (13025, DeribitErrorCode::MethodSwitchedOffByAdmin),
            (13028, DeribitErrorCode::TemporarilyUnavailable),
            (13030, DeribitErrorCode::MmpTrigger),
            (13031, DeribitErrorCode::VerificationRequired),
            (13032, DeribitErrorCode::NonUniqueOrderLabel),
            (13034, DeribitErrorCode::NoMoreSecurityKeysAllowed),
            (13035, DeribitErrorCode::ActiveComboLimitReached),
            (13036, DeribitErrorCode::UnavailableForComboBooks),
            (13037, DeribitErrorCode::IncompleteKycData),
            (13040, DeribitErrorCode::MmpRequired),
            (13042, DeribitErrorCode::CodNotEnabled),
            (13043, DeribitErrorCode::QuotesFrozen),
            (13403, DeribitErrorCode::ScopeExceeded),
            (13503, DeribitErrorCode::Unavailable),
            (13666, DeribitErrorCode::RequestCancelledByUser),
            (13777, DeribitErrorCode::Replaced),
            (
                13778,
                DeribitErrorCode::RawSubscriptionsNotAvailableForUnauthorized,
            ),
            (13780, DeribitErrorCode::MovePositionsOverLimit),
            (13781, DeribitErrorCode::CouponAlreadyUsed),
            (13791, DeribitErrorCode::KycTransferAlreadyInitiated),
        ];

        for (code, expected_error) in test_cases {
            // Test conversion from i32 to DeribitErrorCode
            let error = DeribitErrorCode::from(code);
            assert_eq!(error, expected_error, "Failed for code {}", code);

            // Test conversion from DeribitErrorCode to i32
            assert_eq!(
                error.code(),
                code,
                "Failed reverse conversion for {:?}",
                expected_error
            );
        }
    }

    #[test]
    fn test_all_error_messages() {
        // Test that all error codes have proper messages
        let error_codes = vec![
            DeribitErrorCode::Success,
            DeribitErrorCode::AuthorizationRequired,
            DeribitErrorCode::Error,
            DeribitErrorCode::QtyTooLow,
            DeribitErrorCode::OrderOverlap,
            DeribitErrorCode::OrderNotFound,
            DeribitErrorCode::PriceTooLow,
            DeribitErrorCode::PriceTooLow4Idx,
            DeribitErrorCode::PriceTooHigh,
            DeribitErrorCode::NotEnoughFunds,
            DeribitErrorCode::AlreadyClosed,
            DeribitErrorCode::PriceNotAllowed,
            DeribitErrorCode::BookClosed,
            DeribitErrorCode::PmeMaxTotalOpenOrders,
            DeribitErrorCode::PmeMaxFutureOpenOrders,
            DeribitErrorCode::PmeMaxOptionOpenOrders,
            DeribitErrorCode::PmeMaxFutureOpenOrdersSize,
            DeribitErrorCode::PmeMaxOptionOpenOrdersSize,
            DeribitErrorCode::NonPmeMaxFuturePositionSize,
            DeribitErrorCode::LockedByAdmin,
            DeribitErrorCode::InvalidOrUnsupportedInstrument,
            DeribitErrorCode::InvalidAmount,
            DeribitErrorCode::InvalidQuantity,
            DeribitErrorCode::InvalidPrice,
            DeribitErrorCode::InvalidMaxShow,
            DeribitErrorCode::InvalidOrderId,
            DeribitErrorCode::PricePrecisionExceeded,
            DeribitErrorCode::NonIntegerContractAmount,
            DeribitErrorCode::TooManyRequests,
            DeribitErrorCode::NotOwnerOfOrder,
            DeribitErrorCode::MustBeWebsocketRequest,
            DeribitErrorCode::InvalidArgsForInstrument,
            DeribitErrorCode::WholeCostTooLow,
            DeribitErrorCode::NotImplemented,
            DeribitErrorCode::TriggerPriceTooHigh,
            DeribitErrorCode::TriggerPriceTooLow,
            DeribitErrorCode::InvalidMaxShowAmount,
            DeribitErrorCode::NonPmeTotalShortOptionsPositionsSize,
            DeribitErrorCode::PmeMaxRiskReducingOrders,
            DeribitErrorCode::NotEnoughFundsInCurrency,
            DeribitErrorCode::Retry,
            DeribitErrorCode::SettlementInProgress,
            DeribitErrorCode::PriceWrongTick,
            DeribitErrorCode::TriggerPriceWrongTick,
            DeribitErrorCode::CanNotCancelLiquidationOrder,
            DeribitErrorCode::CanNotEditLiquidationOrder,
            DeribitErrorCode::MatchingEngineQueueFull,
            DeribitErrorCode::NotOnThisServer,
            DeribitErrorCode::CancelOnDisconnectFailed,
            DeribitErrorCode::TooManyConcurrentRequests,
            DeribitErrorCode::DisabledWhilePositionLock,
            DeribitErrorCode::AlreadyFilled,
            DeribitErrorCode::MaxSpotOpenOrders,
            DeribitErrorCode::PostOnlyPriceModificationNotPossible,
            DeribitErrorCode::MaxSpotOrderQuantity,
            DeribitErrorCode::InvalidArguments,
            DeribitErrorCode::OtherReject,
            DeribitErrorCode::OtherError,
            DeribitErrorCode::NoMoreTriggers,
            DeribitErrorCode::InvalidTriggerPrice,
            DeribitErrorCode::OutdatedInstrumentForIvOrder,
            DeribitErrorCode::NoAdvForFutures,
            DeribitErrorCode::NoAdvPostonly,
            DeribitErrorCode::NotAdvOrder,
            DeribitErrorCode::PermissionDenied,
            DeribitErrorCode::BadArgument,
            DeribitErrorCode::NotOpenOrder,
            DeribitErrorCode::InvalidEvent,
            DeribitErrorCode::OutdatedInstrument,
            DeribitErrorCode::UnsupportedArgCombination,
            DeribitErrorCode::WrongMaxShowForOption,
            DeribitErrorCode::BadArguments,
            DeribitErrorCode::BadRequest,
            DeribitErrorCode::SystemMaintenance,
            DeribitErrorCode::SubscribeErrorUnsubscribed,
            DeribitErrorCode::TransferNotFound,
            DeribitErrorCode::PostOnlyReject,
            DeribitErrorCode::PostOnlyNotAllowed,
            DeribitErrorCode::UnauthenticatedPublicRequestsTemporarilyDisabled,
            DeribitErrorCode::InvalidAddr,
            DeribitErrorCode::InvalidTransferAddress,
            DeribitErrorCode::AddressAlreadyExist,
            DeribitErrorCode::MaxAddrCountExceeded,
            DeribitErrorCode::InternalServerError,
            DeribitErrorCode::DisabledDepositAddressCreation,
            DeribitErrorCode::AddressBelongsToUser,
            DeribitErrorCode::NoDepositAddress,
            DeribitErrorCode::AccountLocked,
            DeribitErrorCode::TooManySubaccounts,
            DeribitErrorCode::WrongSubaccountName,
            DeribitErrorCode::LoginOverLimit,
            DeribitErrorCode::RegistrationOverLimit,
            DeribitErrorCode::CountryIsBanned,
            DeribitErrorCode::TransferNotAllowed,
            DeribitErrorCode::SecurityKeyAuthorizationOverLimit,
            DeribitErrorCode::InvalidCredentials,
            DeribitErrorCode::PwdMatchError,
            DeribitErrorCode::SecurityError,
            DeribitErrorCode::UserNotFound,
            DeribitErrorCode::RequestFailed,
            DeribitErrorCode::Unauthorized,
            DeribitErrorCode::ValueRequired,
            DeribitErrorCode::ValueTooShort,
            DeribitErrorCode::UnavailableInSubaccount,
            DeribitErrorCode::InvalidPhoneNumber,
            DeribitErrorCode::CannotSendSms,
            DeribitErrorCode::InvalidSmsCode,
            DeribitErrorCode::InvalidInput,
            DeribitErrorCode::InvalidContentType,
            DeribitErrorCode::OrderbookClosed,
            DeribitErrorCode::NotFound,
            DeribitErrorCode::Forbidden,
            DeribitErrorCode::MethodSwitchedOffByAdmin,
            DeribitErrorCode::TemporarilyUnavailable,
            DeribitErrorCode::MmpTrigger,
            DeribitErrorCode::VerificationRequired,
            DeribitErrorCode::NonUniqueOrderLabel,
            DeribitErrorCode::NoMoreSecurityKeysAllowed,
            DeribitErrorCode::ActiveComboLimitReached,
            DeribitErrorCode::UnavailableForComboBooks,
            DeribitErrorCode::IncompleteKycData,
            DeribitErrorCode::MmpRequired,
            DeribitErrorCode::CodNotEnabled,
            DeribitErrorCode::QuotesFrozen,
            DeribitErrorCode::ScopeExceeded,
            DeribitErrorCode::Unavailable,
            DeribitErrorCode::RequestCancelledByUser,
            DeribitErrorCode::Replaced,
            DeribitErrorCode::RawSubscriptionsNotAvailableForUnauthorized,
            DeribitErrorCode::MovePositionsOverLimit,
            DeribitErrorCode::CouponAlreadyUsed,
            DeribitErrorCode::KycTransferAlreadyInitiated,
            DeribitErrorCode::Unknown(12345),
        ];

        for error_code in error_codes {
            let message = error_code.message();
            assert!(
                !message.is_empty(),
                "Message should not be empty for {:?}",
                error_code
            );
            assert!(
                !message.is_empty(),
                "Message should have content for {:?}",
                error_code
            );
        }
    }

    #[test]
    fn test_comprehensive_error_categorization() {
        // Test authorization errors
        let auth_errors = vec![
            DeribitErrorCode::AuthorizationRequired,
            DeribitErrorCode::Unauthorized,
        ];
        for error in auth_errors {
            assert!(
                error.is_authorization_error(),
                "{:?} should be authorization error",
                error
            );
            assert!(!error.is_success(), "{:?} should not be success", error);
            assert!(
                !error.is_rate_limit_error(),
                "{:?} should not be rate limit error",
                error
            );
            assert!(
                !error.is_validation_error(),
                "{:?} should not be validation error",
                error
            );
            assert!(
                !error.is_trading_error(),
                "{:?} should not be trading error",
                error
            );
        }

        // Test rate limit errors
        let rate_limit_errors = vec![
            DeribitErrorCode::TooManyRequests,
            DeribitErrorCode::TooManyConcurrentRequests,
        ];
        for error in rate_limit_errors {
            assert!(
                error.is_rate_limit_error(),
                "{:?} should be rate limit error",
                error
            );
            assert!(!error.is_success(), "{:?} should not be success", error);
            assert!(
                !error.is_authorization_error(),
                "{:?} should not be authorization error",
                error
            );
            assert!(
                !error.is_validation_error(),
                "{:?} should not be validation error",
                error
            );
            assert!(
                !error.is_trading_error(),
                "{:?} should not be trading error",
                error
            );
        }

        // Test validation errors
        let validation_errors = vec![
            DeribitErrorCode::InvalidAmount,
            DeribitErrorCode::InvalidPrice,
            DeribitErrorCode::InvalidQuantity,
            DeribitErrorCode::InvalidOrderId,
            DeribitErrorCode::InvalidArguments,
            DeribitErrorCode::BadArgument,
            DeribitErrorCode::BadArguments,
            DeribitErrorCode::InvalidInput,
        ];
        for error in validation_errors {
            assert!(
                error.is_validation_error(),
                "{:?} should be validation error",
                error
            );
            assert!(!error.is_success(), "{:?} should not be success", error);
            assert!(
                !error.is_authorization_error(),
                "{:?} should not be authorization error",
                error
            );
            assert!(
                !error.is_rate_limit_error(),
                "{:?} should not be rate limit error",
                error
            );
            assert!(
                !error.is_trading_error(),
                "{:?} should not be trading error",
                error
            );
        }

        // Test trading errors
        let trading_errors = vec![
            DeribitErrorCode::QtyTooLow,
            DeribitErrorCode::OrderOverlap,
            DeribitErrorCode::OrderNotFound,
            DeribitErrorCode::PriceTooLow,
            DeribitErrorCode::PriceTooHigh,
            DeribitErrorCode::NotEnoughFunds,
            DeribitErrorCode::AlreadyClosed,
            DeribitErrorCode::PriceNotAllowed,
            DeribitErrorCode::BookClosed,
        ];
        for error in trading_errors {
            assert!(
                error.is_trading_error(),
                "{:?} should be trading error",
                error
            );
            assert!(!error.is_success(), "{:?} should not be success", error);
            assert!(
                !error.is_authorization_error(),
                "{:?} should not be authorization error",
                error
            );
            assert!(
                !error.is_rate_limit_error(),
                "{:?} should not be rate limit error",
                error
            );
            assert!(
                !error.is_validation_error(),
                "{:?} should not be validation error",
                error
            );
        }

        // Test success
        let success = DeribitErrorCode::Success;
        assert!(success.is_success(), "Success should be success");
        assert!(
            !success.is_authorization_error(),
            "Success should not be authorization error"
        );
        assert!(
            !success.is_rate_limit_error(),
            "Success should not be rate limit error"
        );
        assert!(
            !success.is_validation_error(),
            "Success should not be validation error"
        );
        assert!(
            !success.is_trading_error(),
            "Success should not be trading error"
        );
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = DeribitErrorCode::AuthorizationRequired;
        let error_trait: &dyn std::error::Error = &error;
        assert!(error_trait.source().is_none());
    }

    #[test]
    fn test_clone_and_equality() {
        let error1 = DeribitErrorCode::AuthorizationRequired;
        let error2 = error1.clone();
        assert_eq!(error1, error2);

        let error3 = DeribitErrorCode::Error;
        assert_ne!(error1, error3);

        let unknown1 = DeribitErrorCode::Unknown(12345);
        let unknown2 = DeribitErrorCode::Unknown(12345);
        let unknown3 = DeribitErrorCode::Unknown(54321);
        assert_eq!(unknown1, unknown2);
        assert_ne!(unknown1, unknown3);
    }

    #[test]
    fn test_hash_implementation() {
        use std::collections::HashSet;

        let mut error_set = HashSet::new();
        error_set.insert(DeribitErrorCode::AuthorizationRequired);
        error_set.insert(DeribitErrorCode::Error);
        error_set.insert(DeribitErrorCode::AuthorizationRequired); // Duplicate

        assert_eq!(error_set.len(), 2); // Should only contain 2 unique errors
        assert!(error_set.contains(&DeribitErrorCode::AuthorizationRequired));
        assert!(error_set.contains(&DeribitErrorCode::Error));
        assert!(!error_set.contains(&DeribitErrorCode::Success));
    }

    #[test]
    fn test_display_and_debug_implementations() {
        let error = DeribitErrorCode::AuthorizationRequired;
        let display_str = format!("{}", error);
        let debug_str = format!("{:?}", error);

        // Both should contain some representation of the error
        assert!(!display_str.is_empty());
        assert!(!debug_str.is_empty());

        // Test unknown error
        let unknown_error = DeribitErrorCode::Unknown(99999);
        let unknown_display = format!("{}", unknown_error);
        let unknown_debug = format!("{:?}", unknown_error);

        assert!(!unknown_display.is_empty());
        assert!(!unknown_debug.is_empty());
    }
}
