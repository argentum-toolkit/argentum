use crate::generator::api_doc::{
    OpenApiGenerator, PathsGenerator, RequestGenerator, ResponseGenerator, TypeGenerator,
};
use crate::generator::business::{
    BusinessCargoGenerator, BusinessDiGenerator, BusinessEntityGenerator,
    BusinessGitIgnoreGenerator, BusinessLibGenerator, BusinessMockGenerator,
    BusinessReadmeGenerator, BusinessRepositoryGenerator,
};
use crate::generator::devops::{DockerComposeDevGenerator, DockerComposeGenerator};
use crate::generator::fe_app::{
    FeAppGenerator, FeBuildRsGenerator, FeCargoGenerator, FeGitIgnoreGenerator, FeMainGenerator,
    FeReadmeAdocGenerator, FeRouteGenerator, FeRsxGenerator, FeStyleGenerator,
};
use crate::generator::infrastructure::{
    InfAdapterGenerator, InfCargoGenerator, InfDbModGenerator, InfDiGenerator,
    InfGitIgnoreGenerator, InfLibGenerator, InfMigrationGenerator, InfReadmeGenerator,
};
use crate::generator::ui::{
    UiCargoGenerator, UiLibGenerator, UiPageGenerator, UiPageModGenerator, UiReadmeGenerator,
    UiRsxModGenerator,
};
use crate::generator::{
    ApiDocGenerator, BusinessGenerator, ComponentGenerator, DbInfrastructureGenerator,
    DevopsGenerator, FileExistenceChecker, FileGenerator, InfrastructureGenerator, UiGenerator,
};
use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::di::template_infrastructure_di_factory;
use std::error::Error;
use std::sync::Arc;

pub struct Di<L>
where
    L: LoggerTrait,
{
    // Public services
    pub api_doc_generator: Arc<ApiDocGenerator<L>>,
    pub business_generator: Arc<BusinessGenerator<L>>,
    pub db_infrastructure_generator: Arc<DbInfrastructureGenerator<L>>,
    pub devops_generator: Arc<DevopsGenerator<L>>,
    pub fe_app_generator: Arc<FeAppGenerator<L>>,
    pub infrastructure_generator: Arc<InfrastructureGenerator<L>>,
    pub ui_generator: Arc<UiGenerator<L>>,
    pub component_generator: Arc<ComponentGenerator<L>>,
}

impl<L> Di<L>
where
    L: LoggerTrait,
{
    pub fn new(
        api_doc_generator: Arc<ApiDocGenerator<L>>,
        business_generator: Arc<BusinessGenerator<L>>,
        db_infrastructure_generator: Arc<DbInfrastructureGenerator<L>>,
        devops_generator: Arc<DevopsGenerator<L>>,
        fe_app_generator: Arc<FeAppGenerator<L>>,
        infrastructure_generator: Arc<InfrastructureGenerator<L>>,
        ui_generator: Arc<UiGenerator<L>>,
        component_generator: Arc<ComponentGenerator<L>>,
    ) -> Self {
        Self {
            api_doc_generator,
            business_generator,
            db_infrastructure_generator,
            devops_generator,
            fe_app_generator,
            infrastructure_generator,
            ui_generator,
            component_generator,
        }
    }
}

pub fn di_factory<L>(logger: Arc<L>) -> Result<Di<L>, Box<dyn Error>>
where
    L: LoggerTrait + 'static,
{
    let templates = vec![
        (
            "api_doc/openapi.yaml",
            include_str!("../template/api_doc/openapi.yaml.hbs"),
        ),
        (
            "api_doc/paths.oas.yaml",
            include_str!("../template/api_doc/paths.oas.yaml.hbs"),
        ),
        (
            "api_doc/request.oas.yaml",
            include_str!("../template/api_doc/request.oas.yaml.hbs"),
        ),
        (
            "api_doc/response.oas.yaml",
            include_str!("../template/api_doc/response.oas.yaml.hbs"),
        ),
        (
            "api_doc/type.oas.yaml",
            include_str!("../template/api_doc/type.oas.yaml.hbs"),
        ),
        (
            "ui/Cargo.toml",
            include_str!("../template/ui/Cargo.toml.hbs"),
        ),
        (
            "ui/readme.adoc",
            include_str!("../template/ui/readme.adoc.hbs"),
        ),
        ("ui/src/lib", include_str!("../template/ui/src/lib.hbs")),
        (
            "ui/src/rsx/mod",
            include_str!("../template/ui/src/rsx/mod.hbs"),
        ),
        (
            "ui/src/rsx/page/mod",
            include_str!("../template/ui/src/rsx/page/mod.hbs"),
        ),
        (
            "ui/src/rsx/page/page",
            include_str!("../template/ui/src/rsx/page/page.hbs"),
        ),
        (
            "devops/docker-compose.yaml",
            include_str!("../template/devops/docker-compose.yaml.hbs"),
        ),
        (
            "devops/docker-compose.dev.yaml",
            include_str!("../template/devops/docker-compose.dev.yaml.hbs"),
        ),
        (
            "infrastructure/db_dto",
            include_str!("../template/infrastructure/db_dto.hbs"),
        ),
        (
            "infrastructure/db_repository",
            include_str!("../template/infrastructure/db_repository.hbs"),
        ),
        (
            "infrastructure/Cargo.toml",
            include_str!("../template/infrastructure/Cargo.toml.hbs"),
        ),
        (
            "infrastructure/.gitignore",
            include_str!("../template/infrastructure/.gitignore.hbs"),
        ),
        (
            "infrastructure/readme.adoc",
            include_str!("../template/infrastructure/readme.adoc.hbs"),
        ),
        (
            "infrastructure/src/lib",
            include_str!("../template/infrastructure/src/lib.hbs"),
        ),
        (
            "infrastructure/src/di",
            include_str!("../template/infrastructure/src/di.hbs"),
        ),
        (
            "infrastructure/src/db/mod",
            include_str!("../template/infrastructure/src/db/mod.hbs"),
        ),
        (
            "infrastructure/src/db/adapter",
            include_str!("../template/infrastructure/src/db/adapter.hbs"),
        ),
        (
            "infrastructure/src/db/migration/mod",
            include_str!("../template/infrastructure/src/db/migration/mod.hbs"),
        ),
        (
            "infrastructure/src/db/migration/migration_000_init",
            include_str!("../template/infrastructure/src/db/migration/migration_000_init.hbs"),
        ),
        (
            "fe_app/Cargo.toml",
            include_str!("../template/fe_app/Cargo.toml.hbs"),
        ),
        (
            "fe_app/build.rs",
            include_str!("../template/fe_app/build.rs.hbs"),
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
            "fe_app/src/rsx/page_mod",
            include_str!("../template/fe_app/src/rsx/page_mod.hbs"),
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
        (
            "business/Cargo.toml",
            include_str!("../template/business/Cargo.toml.hbs"),
        ),
        (
            "business/.gitignore",
            include_str!("../template/business/.gitignore.hbs"),
        ),
        (
            "business/readme.adoc",
            include_str!("../template/business/readme.adoc.hbs"),
        ),
        (
            "business/src/lib",
            include_str!("../template/business/src/lib.hbs"),
        ),
        (
            "business/src/di",
            include_str!("../template/business/src/di.hbs"),
        ),
        (
            "business/src/entity",
            include_str!("../template/business/src/entity.hbs"),
        ),
        (
            "business/src/repository",
            include_str!("../template/business/src/repository.hbs"),
        ),
        (
            "business/src/mock_repository",
            include_str!("../template/business/src/mock_repository.hbs"),
        ),
    ];

    let template_di = template_infrastructure_di_factory(templates)?;

    //services
    let file_existence_checker = Arc::new(FileExistenceChecker);

    let file_generator = Arc::new(FileGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let db_infrastructure_generator = Arc::new(DbInfrastructureGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let docker_compose_generator = Arc::new(DockerComposeGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let docker_compose_dev_generator = Arc::new(DockerComposeDevGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let devops_generator = Arc::new(DevopsGenerator::new(
        logger.clone(),
        docker_compose_generator,
        docker_compose_dev_generator,
    ));

    let fe_build_rs_generator = Arc::new(FeBuildRsGenerator::new(file_generator.clone()));

    let fe_cargo_generator = Arc::new(FeCargoGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
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
        fe_build_rs_generator,
        fe_cargo_generator,
        style_generator,
        gitignore_generator,
        readme_adoc_generator,
        rsx_generator,
        main_generator,
        route_generator,
    ));

    let business_cargo_generator = Arc::new(BusinessCargoGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let business_git_ignore_generator =
        Arc::new(BusinessGitIgnoreGenerator::new(file_generator.clone()));

    let business_readme_generator = Arc::new(BusinessReadmeGenerator::new(file_generator.clone()));

    let business_lib_generator = Arc::new(BusinessLibGenerator::new(file_generator.clone()));

    let business_entity_generator = Arc::new(BusinessEntityGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let business_repository_generator = Arc::new(BusinessRepositoryGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let business_mock_generator = Arc::new(BusinessMockGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let business_di_generator = Arc::new(BusinessDiGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let inf_cargo_generator = Arc::new(InfCargoGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let inf_git_ignore_generator = Arc::new(InfGitIgnoreGenerator::new(file_generator.clone()));

    let inf_readme_generator = Arc::new(InfReadmeGenerator::new(file_generator.clone()));

    let inf_lib_generator = Arc::new(InfLibGenerator::new(file_generator.clone()));

    let inf_di_generator = Arc::new(InfDiGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let inf_db_mod_generator = Arc::new(InfDbModGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let inf_adapter_generator = Arc::new(InfAdapterGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let inf_migration_generator = Arc::new(InfMigrationGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let infrastructure_generator = Arc::new(InfrastructureGenerator::new(
        logger.clone(),
        inf_cargo_generator,
        inf_git_ignore_generator,
        inf_readme_generator,
        inf_lib_generator,
        inf_di_generator,
        inf_db_mod_generator,
        inf_adapter_generator,
        inf_migration_generator,
        db_infrastructure_generator.clone(),
    ));

    let business_generator = Arc::new(BusinessGenerator::new(
        logger.clone(),
        business_cargo_generator,
        business_git_ignore_generator,
        business_readme_generator,
        business_lib_generator,
        business_entity_generator,
        business_repository_generator,
        business_mock_generator,
        business_di_generator,
    ));

    let api_doc_open_api_generator = Arc::new(OpenApiGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let api_doc_paths_generator = Arc::new(PathsGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let api_doc_request_generator = Arc::new(RequestGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let api_doc_response_generator = Arc::new(ResponseGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let api_doc_type_generator = Arc::new(TypeGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let api_doc_generator = Arc::new(ApiDocGenerator::new(
        logger.clone(),
        api_doc_open_api_generator,
        api_doc_paths_generator,
        api_doc_request_generator,
        api_doc_response_generator,
        api_doc_type_generator,
    ));

    let ui_cargo_generator = Arc::new(UiCargoGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let ui_readme_generator = Arc::new(UiReadmeGenerator::new(file_generator.clone()));

    let ui_lib_generator = Arc::new(UiLibGenerator::new(file_generator.clone()));

    let ui_rsx_mod_generator = Arc::new(UiRsxModGenerator::new(file_generator.clone()));

    let ui_page_mod_generator = Arc::new(UiPageModGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let ui_page_generator = Arc::new(UiPageGenerator::new(
        logger.clone(),
        template_di.renderer.clone(),
        file_existence_checker.clone(),
    ));

    let ui_generator = Arc::new(UiGenerator::new(
        logger.clone(),
        ui_cargo_generator,
        ui_readme_generator,
        ui_lib_generator,
        ui_rsx_mod_generator,
        ui_page_mod_generator,
        ui_page_generator,
    ));

    let component_generator = Arc::new(ComponentGenerator::new(
        logger.clone(),
        business_generator.clone(),
        api_doc_generator.clone(),
        devops_generator.clone(),
        infrastructure_generator.clone(),
        ui_generator.clone(),
    ));

    Ok(Di::new(
        api_doc_generator,
        business_generator,
        db_infrastructure_generator,
        devops_generator,
        fe_app_generator,
        infrastructure_generator,
        ui_generator,
        component_generator,
    ))
}
