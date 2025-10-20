use crate::generator::fe_app::{
    FeAppGenerator, FeCargoGenerator, FeGitIgnoreGenerator, FeMainGenerator, FeReadmeAdocGenerator,
    FeRouteGenerator, FeRsxGenerator, FeStyleGenerator,
};
use crate::generator::{DbInfrastructureGenerator, FileGenerator};
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_template_infrastructure::di::template_infrastructure_di_factory;
use std::error::Error;
use std::sync::Arc;

pub(crate) struct DiC<L>
where
    L: LoggerTrait,
{
    // Public services
    pub(crate) db_infrastructure_generator: Arc<DbInfrastructureGenerator<L>>,
    pub(crate) fe_app_generator: Arc<FeAppGenerator<L>>,
}

impl<L> DiC<L>
where
    L: LoggerTrait,
{
    pub fn new(
        db_infrastructure_generator: Arc<DbInfrastructureGenerator<L>>,
        fe_app_generator: Arc<FeAppGenerator<L>>,
    ) -> Self {
        Self {
            db_infrastructure_generator,
            fe_app_generator,
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
        (
            "fe_app/Cargo.toml",
            include_str!("../template/fe_app/Cargo.toml.hbs"),
        ),
        (
            "fe_app/input.css",
            include_str!("../template/fe_app/input.css.hbs"),
        ),
        (
            "fe_app/package.json",
            include_str!("../template/fe_app/package.json.hbs"),
        ),
        (
            "fe_app/tailwind.config.js",
            include_str!("../template/fe_app/tailwind.config.js.hbs"),
        ),
        (
            "fe_app/.gitignore",
            include_str!("../template/fe_app/.gitignore.hbs"),
        ),
        (
            "fe_app/readme.adoc",
            include_str!("../template/fe_app/readme.adoc.hbs"),
        ),
        (
            "fe_app/src/rsx/element_mod",
            include_str!("../template/fe_app/src/rsx/element_mod.hbs"),
        ),
        (
            "fe_app/src/rsx/element_footer",
            include_str!("../template/fe_app/src/rsx/element_footer.hbs"),
        ),
        (
            "fe_app/src/rsx/element_navbar",
            include_str!("../template/fe_app/src/rsx/element_navbar.hbs"),
        ),
        (
            "fe_app/src/rsx/page_mod",
            include_str!("../template/fe_app/src/rsx/page_mod.hbs"),
        ),
        (
            "fe_app/src/rsx/page_home",
            include_str!("../template/fe_app/src/rsx/page_home.hbs"),
        ),
        (
            "fe_app/src/rsx/app",
            include_str!("../template/fe_app/src/rsx/app.hbs"),
        ),
        (
            "fe_app/src/rsx/mod",
            include_str!("../template/fe_app/src/rsx/mod.hbs"),
        ),
        (
            "fe_app/src/rsx/wrapper",
            include_str!("../template/fe_app/src/rsx/wrapper.hbs"),
        ),
        (
            "fe_app/src/main",
            include_str!("../template/fe_app/src/main.hbs"),
        ),
        (
            "fe_app/src/route",
            include_str!("../template/fe_app/src/route.hbs"),
        ),
    ];

    let template_di = template_infrastructure_di_factory(templates)?;

    //services
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let file_generator = Arc::new(FileGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
    ));

    let db_infrastructure_generator = Arc::new(DbInfrastructureGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
    ));

    let fe_cargo_generator = Arc::new(FeCargoGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
    ));

    let style_generator = Arc::new(FeStyleGenerator::new(
        logger.clone(),
        file_generator.clone(),
    ));

    let gitignore_generator = Arc::new(FeGitIgnoreGenerator::new(file_generator.clone()));

    let readme_adoc_generator = Arc::new(FeReadmeAdocGenerator::new(file_generator.clone()));

    let rsx_generator = Arc::new(FeRsxGenerator::new(logger.clone(), file_generator.clone()));

    let main_generator = Arc::new(FeMainGenerator::new(file_generator.clone()));
    let route_generator = Arc::new(FeRouteGenerator::new(file_generator.clone()));

    let fe_app_generator = Arc::new(FeAppGenerator::new(
        logger.clone(),
        fe_cargo_generator,
        style_generator,
        gitignore_generator,
        readme_adoc_generator,
        rsx_generator,
        main_generator,
        route_generator,
    ));

    Ok(DiC::<DefaultLogger<PrettyWriter>>::new(
        db_infrastructure_generator,
        fe_app_generator,
    ))
}
