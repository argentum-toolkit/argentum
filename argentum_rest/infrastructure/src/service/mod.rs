mod bearer_authenticator;
mod error_handler;
pub mod error_pre_handler;
mod header_params_extractor;
mod path_params_extractor;
mod query_params_extractor;
mod request_transformer;
mod response_transformer;
mod router;
mod router_combinator;
mod schema_extractor;
mod server;
mod validation_error_transformer;

pub use bearer_authenticator::BearerAuthenticator;
pub use error_handler::ErrorHandler;
pub use error_pre_handler::ErrorPreHandler;
pub use header_params_extractor::HeaderParamsExtractor;
pub use path_params_extractor::PathParamsExtractor;
pub use query_params_extractor::QueryParamsExtractor;
pub use request_transformer::RequestTransformer;
pub use response_transformer::ResponseToJsonTransformer;
pub use router::RouterTrait;
pub use router_combinator::RouterCombinator;
pub use schema_extractor::SchemaExtractor;
pub use server::Server;
use std::collections::HashMap;
pub use validation_error_transformer::ValidationErrorTransformer;

type RawPathParams<'a> = HashMap<&'a str, &'a str>;
//TODO: Use recursive object instead of HashMap
type RawQueryParams = HashMap<String, String>;
