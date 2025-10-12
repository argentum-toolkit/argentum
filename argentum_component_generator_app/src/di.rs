use crate::generator::DbInfrastructureGenerator;
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_template_infrastructure::di::template_infrastructure_di_factory;
use std::error::Error;
use std::sync::Arc;

pub struct DiC<L>
where
    L: LoggerTrait,
{
    // Public services
    pub db_repository_generator: Arc<DbInfrastructureGenerator<L>>,
}

impl<L> DiC<L>
where
    L: LoggerTrait,
{
    pub fn new(db_repository_generator: Arc<DbInfrastructureGenerator<L>>) -> Self {
        Self {
            db_repository_generator,
        }
    }
}

pub fn di_factory() -> Result<DiC<DefaultLogger<PrettyWriter>>, Box<dyn Error>> {
    let templates = vec![
        (
            "infrastructure/db_dto",
            include_str!("../template/infrastructure/db_dto.hbs"),
        ),
        (
            "infrastructure/db_repository",
            include_str!("../template/infrastructure/db_repository.hbs"),
        ),
    ];

    let template_di = template_infrastructure_di_factory(templates)?;

    //services
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));
    // let renderer = Arc::new(Renderer::new(Arc::new(reg)));
    let db_repository_generator = Arc::new(DbInfrastructureGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
    ));

    Ok(DiC::<DefaultLogger<PrettyWriter>>::new(
        db_repository_generator,
    ))
}
