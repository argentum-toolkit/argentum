use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::{DefaultLogger, Level};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::mock::StdoutNotificator;
use argentum_rest_infrastructure::service::Server;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use std::net::SocketAddr;

use argentum_rest_infrastructure::RestDiC;
use argentum_user_account_infrastructure::api::ApiDiC;
use argentum_user_account_infrastructure::di::UserAccountInfrastructureDiCBuilder;
use argentum_user_infrastructure::di::UserDiCBuilder;
use std::sync::Arc;

pub struct DiC {
    // Public services
    pub server: Arc<Server>,
}

impl DiC {
    pub fn new(server: Arc<Server>) -> DiC {
        DiC { server }
    }
}

pub fn di_factory() -> DiC {
    let user_account_pg_connection_pool_manager =
        Arc::new(ConnectionPoolManager::new("AG_USER_ACCOUNT_DATABASE_URL"));

    let user_pg_connection_pool_manager =
        Arc::new(ConnectionPoolManager::new("AG_USER_DATABASE_URL"));

    let unique_id_factory = Arc::new(UniqueIdFactory::new());

    let u_di = UserDiCBuilder::new()
        .defalt_services(user_pg_connection_pool_manager, unique_id_factory.clone())
        .build();

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let pbkdf2_password_encryptor = Arc::new(Pbkdf2::new());

    let notificator = Arc::new(StdoutNotificator::new());

    let ua_di = UserAccountInfrastructureDiCBuilder::new(
        unique_id_factory.clone(),
        pbkdf2_password_encryptor.clone(),
        pbkdf2_password_encryptor,
        logger.clone(),
        notificator,
    )
    .services(
        unique_id_factory,
        user_account_pg_connection_pool_manager,
        u_di.business_dic.anonymous_user_repository,
        u_di.business_dic.authenticated_user_repository,
        u_di.business_dic.anonymous_binding_repository,
    )
    .config(
        "Argentum ToolKit demo web application".to_string(),
        3600, // TTL 1h
        "http://localhost:8082/change-password/".to_string(),
    )
    .build();

    let rest_di = RestDiC::new(logger.clone(), ua_di.user_authenticates_with_token_uc);

    let api_di = ApiDiC::new(
        rest_di.request_transformer,
        rest_di.bearer_authenticator,
        ua_di.anonymous_registers_handler,
        ua_di.user_logins_with_password_handler,
        ua_di.user_registers_with_password_handler,
        ua_di.anonymous_requests_restore_token_handler,
        ua_di.anonymous_with_token_changes_password_handler,
        rest_di.error_pre_handler,
    );

    // let listen = "172.18.0.1:8080";
    // let listen = "127.0.0.1:8080";
    let listen = "0.0.0.0:8080";
    let addr: SocketAddr = listen.parse().expect("Unable to parse socket address");

    let server = Arc::new(Server::new(
        addr,
        api_di.router,
        rest_di.response_transformer,
        rest_di.error_handler,
        logger,
    ));

    DiC::new(server)
}
