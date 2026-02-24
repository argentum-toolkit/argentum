use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_user_account_business::use_case::guest_registers::GuestRegistersUc;
use argentum_user_account_business::use_case::member_logins_with_password::MemberLoginsWithPasswordUc;
use argentum_user_account_business::use_case::member_registers_with_password::MemberRegistersWithPasswordUc;
use argentum_user_business::data_type::builder::NameBuilder;
use argentum_user_business::entity::user::Guest;
use argentum_user_business::use_case::member_authenticates_with_token::MemberAuthenticatesWithTokenUc;
use std::sync::Arc;

pub struct App<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    id_factory: Arc<dyn IdFactory<Id = I>>,
    guest_registers_uc: Arc<GuestRegistersUc<I>>,
    member_logins_with_pw: Arc<MemberLoginsWithPasswordUc<I, L>>,
    member_registers_with_pw: Arc<MemberRegistersWithPasswordUc<I>>,
    user_authenticates_with_token: Arc<MemberAuthenticatesWithTokenUc<I>>,
    logger: Arc<L>,
}

impl<I, L> App<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        id_factory: Arc<dyn IdFactory<Id = I>>,
        guest_registers_uc: Arc<GuestRegistersUc<I>>,
        member_logins_with_pw: Arc<MemberLoginsWithPasswordUc<I, L>>,
        member_registers_with_pw: Arc<MemberRegistersWithPasswordUc<I>>,
        user_authenticates_with_token: Arc<MemberAuthenticatesWithTokenUc<I>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            id_factory,
            guest_registers_uc,
            member_logins_with_pw,
            member_registers_with_pw,
            user_authenticates_with_token,
            logger,
        }
    }

    pub async fn run(&self) -> Result<(), String> {
        self.logger.trace("Demo trace log");
        self.logger.debug("Demo debug log");
        self.logger.info("Demo info log");
        self.logger.warning("Demo warning log");
        self.logger.error("Demo error log");
        self.logger.critical("Demo critical log");

        // events
        pub struct DemoEvent {}
        argentum_event_business::event_boilerplate!(DemoEvent, DemoListenerTrait, DemoDispatcher);

        pub struct DemoListener {}

        impl DemoListenerTrait for DemoListener {
            fn listen(&self, _e: &DemoEvent) {
                println!("Listening demo event")
            }
        }

        let demo_dispatcher = DemoDispatcher::new(vec![DemoListener {}]);
        demo_dispatcher.dispatch(&DemoEvent {});
        //events end

        let anon_id = self.id_factory.create();

        let anon_registration_result = self.guest_registers_uc.execute(anon_id).await;
        let (_, anon_session) = match anon_registration_result {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        let anon_auth_result = self
            .user_authenticates_with_token
            .execute(&anon_session.token)
            .await;

        match anon_auth_result {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
                return Err(e.to_string());
            }
        };

        let user_id = self.id_factory.create();
        let name = NameBuilder::new("Sarah")
            .last(Some("Connor"))
            .try_build()
            .map_err(|e| format!("Wrong name: {}", e.errors.concat()))?;
        let email = EmailAddress::try_new("sarah-connor@example.com")
            .map_err(|e| format!("Wrong email address: {}", e.errors.concat()))?;
        let password = "111";

        let res = self
            .member_registers_with_pw
            .execute(user_id, name, email, password.into(), true)
            .await;
        match res {
            Ok(_) => {
                println!("Registered")
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }

        let anon_id2 = self.id_factory.create();
        let anon2 = Guest::new(anon_id2);
        let email2 = EmailAddress::try_new("sarah-connor@example.com")
            .map_err(|e| format!("Wrong email address: {}", e.errors.concat()))?;
        let password2 = "111";

        let login_result = self
            .member_logins_with_pw
            .execute(Some(anon2), email2, password2.into())
            .await;

        let login = match login_result {
            Ok(l) => {
                println!("Logged In");
                l
            }
            Err(e) => {
                println!("Login error: {}", e);
                return Err(e.to_string());
            }
        };

        let aut_result = self
            .user_authenticates_with_token
            .execute(&login.token)
            .await;

        match aut_result {
            Ok(_) => {
                println!("Authenticated");
                Ok(())
            }
            Err(e) => {
                println!("Error: {}", e);
                Err(e.to_string())
            }
        }
    }
}
