mod component_generator;

pub use component_generator::ComponentGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::api_app::{
        ApiAppApiDiGenerator, ApiAppApiMainGenerator, ApiAppCargoGenerator,
        ApiAppMigrationDiGenerator, ApiAppMigrationMainGenerator, ApiAppReadmeGenerator,
    };
    use crate::generator::api_doc::{
        OpenApiGenerator, PathsGenerator, RequestGenerator, ResponseGenerator, TypeGenerator,
    };
    use crate::generator::business::{
        BusinessCargoGenerator, BusinessDiGenerator, BusinessEntityGenerator,
        BusinessGitIgnoreGenerator, BusinessLibGenerator, BusinessMockGenerator,
        BusinessReadmeGenerator, BusinessRepositoryGenerator,
    };
    use crate::generator::devops::{DockerComposeDevGenerator, DockerComposeGenerator};
    use crate::generator::infrastructure::{
        InfAdapterGenerator, InfCargoGenerator, InfDbModGenerator, InfDiGenerator,
        InfGitIgnoreGenerator, InfLibGenerator, InfMigrationGenerator, InfReadmeGenerator,
    };
    use crate::generator::ui::{
        UiCargoGenerator, UiLibGenerator, UiPageGenerator, UiPageModGenerator, UiReadmeGenerator,
        UiRsxModGenerator,
    };
    use crate::generator::{
        ApiAppGenerator, ApiDocGenerator, BusinessGenerator, DbInfrastructureGenerator,
        DevopsGenerator, FileExistenceChecker, FileGenerator, InfrastructureGenerator,
        RestGenerator, UiGenerator,
    };
    use crate::params::ComponentParams;
    use argentum_log_business::Level;
    use argentum_template_infrastructure::Renderer;
    use handlebars::Handlebars;
    use std::sync::Arc;

    struct TestLogger;

    impl argentum_log_business::LoggerTrait for TestLogger {
        fn log<S: AsRef<str>>(&self, _level: Level, _msg: S) {}
    }

    fn test_component_generator() -> ComponentGenerator<TestLogger> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("t", "x").unwrap();

        let logger = Arc::new(TestLogger);
        let renderer = Arc::new(Renderer::new(Arc::new(handlebars)));
        let file_existence_checker = Arc::new(FileExistenceChecker);
        let file_generator = Arc::new(FileGenerator::new(
            logger.clone(),
            renderer.clone(),
            file_existence_checker.clone(),
        ));

        let business_generator = Arc::new(BusinessGenerator::new(
            logger.clone(),
            Arc::new(BusinessCargoGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(BusinessGitIgnoreGenerator::new(file_generator.clone())),
            Arc::new(BusinessReadmeGenerator::new(file_generator.clone())),
            Arc::new(BusinessLibGenerator::new(file_generator.clone())),
            Arc::new(BusinessEntityGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(BusinessRepositoryGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(BusinessMockGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(BusinessDiGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
        ));

        let api_doc_generator = Arc::new(ApiDocGenerator::new(
            logger.clone(),
            Arc::new(OpenApiGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(PathsGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(RequestGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(ResponseGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(TypeGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
        ));

        let devops_generator = Arc::new(DevopsGenerator::new(
            logger.clone(),
            Arc::new(DockerComposeGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(DockerComposeDevGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
        ));

        let db_infrastructure_generator = Arc::new(DbInfrastructureGenerator::new(
            logger.clone(),
            renderer.clone(),
            file_existence_checker.clone(),
        ));

        let infrastructure_generator = Arc::new(InfrastructureGenerator::new(
            logger.clone(),
            Arc::new(InfCargoGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(InfGitIgnoreGenerator::new(file_generator.clone())),
            Arc::new(InfReadmeGenerator::new(file_generator.clone())),
            Arc::new(InfLibGenerator::new(file_generator.clone())),
            Arc::new(InfDiGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(InfDbModGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(InfAdapterGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(InfMigrationGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            db_infrastructure_generator.clone(),
        ));

        let ui_generator = Arc::new(UiGenerator::new(
            logger.clone(),
            Arc::new(UiCargoGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(UiReadmeGenerator::new(file_generator.clone())),
            Arc::new(UiLibGenerator::new(file_generator.clone())),
            Arc::new(UiRsxModGenerator::new(file_generator.clone())),
            Arc::new(UiPageModGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(UiPageGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
        ));

        let api_app_generator = Arc::new(ApiAppGenerator::new(
            logger.clone(),
            Arc::new(ApiAppCargoGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(ApiAppApiMainGenerator::new(file_generator.clone())),
            Arc::new(ApiAppApiDiGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(ApiAppMigrationMainGenerator::new(file_generator.clone())),
            Arc::new(ApiAppMigrationDiGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(ApiAppReadmeGenerator::new(file_generator.clone())),
        ));

        ComponentGenerator::new(
            logger.clone(),
            business_generator,
            api_doc_generator,
            Arc::new(RestGenerator::new(logger.clone())),
            devops_generator,
            infrastructure_generator,
            ui_generator,
            api_app_generator,
        )
    }

    #[test]
    fn generate_skips_existing_business_cargo_file() {
        let _cwd_guard = crate::generator::test_util::lock_cwd();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let cargo_path = temp_dir
            .path()
            .join("argentum_test_comp/business/Cargo.toml");
        std::fs::create_dir_all(cargo_path.parent().unwrap()).unwrap();
        std::fs::write(&cargo_path, "existing").unwrap();

        let openapi_path = temp_dir
            .path()
            .join("argentum_test_comp/api-doc/openapi.yaml");
        std::fs::create_dir_all(openapi_path.parent().unwrap()).unwrap();
        std::fs::write(
            &openapi_path,
            "openapi: 3.0.0\ninfo:\n  title: test\n  version: 0.0.1\npaths: {}\ncomponents: {}\n",
        )
        .unwrap();

        let result = test_component_generator().generate(&ComponentParams {
            project_prefix: "argentum".to_string(),
            component_name: "test_comp".to_string(),
            entity_name: None,
        });

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&cargo_path).unwrap(), "existing");
    }
}
