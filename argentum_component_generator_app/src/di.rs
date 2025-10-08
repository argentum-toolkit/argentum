use crate::generator::PgRepositoryGenerator;
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
    pub pg_repository_generator: Arc<PgRepositoryGenerator<L>>,
}

impl<L> DiC<L>
where
    L: LoggerTrait,
{
    pub fn new(pg_repository_generator: Arc<PgRepositoryGenerator<L>>) -> Self {
        Self {
            pg_repository_generator,
        }
    }
}

pub fn di_factory() -> Result<DiC<DefaultLogger<PrettyWriter>>, Box<dyn Error>> {
    let templates = vec![(
        "dto/operation_response_enum.item",
        include_str!("../template/infrastructure/pg_repository.hbs"),
    )];

    let template_di = template_infrastructure_di_factory(templates)?;

    //services
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));
    // let renderer = Arc::new(Renderer::new(Arc::new(reg)));
    let pg_repository_generator = Arc::new(PgRepositoryGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
    ));

    Ok(DiC::<DefaultLogger<PrettyWriter>>::new(
        pg_repository_generator,
    ))
}
