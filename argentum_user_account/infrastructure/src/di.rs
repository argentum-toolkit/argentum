use crate::api::server::handler::AnonymousRegistersTrait;
use crate::rest::handler::AnonymousRegistersHandler;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_business::use_case::anonymous_registers::AnonymousRegistersUc;
use std::sync::Arc;

pub struct InfrastructureDiC {
    pub anonymous_registers: Arc<dyn AnonymousRegistersTrait>,
}

impl InfrastructureDiC {
    //todo: pub fn new(domain_di: &DomainDiC) -> Self {
    pub fn new(uc: Arc<AnonymousRegistersUc>, id_factory: Arc<UniqueIdFactory>) -> Self {
        let anonymous_registers = Arc::new(AnonymousRegistersHandler::new(uc, id_factory));

        InfrastructureDiC {
            anonymous_registers,
        }
    }
}
