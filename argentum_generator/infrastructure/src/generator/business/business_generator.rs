use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{
    generator::business::{
        business_cargo_generator::BusinessCargoGenerator,
        business_di_generator::BusinessDiGenerator,
        business_entity_generator::BusinessEntityGenerator,
        business_git_ignore_generator::BusinessGitIgnoreGenerator,
        business_lib_generator::BusinessLibGenerator,
        business_mock_generator::BusinessMockGenerator,
        business_readme_generator::BusinessReadmeGenerator,
        business_repository_generator::BusinessRepositoryGenerator,
    },
    params::BusinessParams,
};

pub struct BusinessGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    cargo: Arc<BusinessCargoGenerator<L>>,
    git_ignore: Arc<BusinessGitIgnoreGenerator<L>>,
    readme: Arc<BusinessReadmeGenerator<L>>,
    lib: Arc<BusinessLibGenerator<L>>,
    entity: Arc<BusinessEntityGenerator<L>>,
    repository: Arc<BusinessRepositoryGenerator<L>>,
    mock: Arc<BusinessMockGenerator<L>>,
    di: Arc<BusinessDiGenerator<L>>,
}

impl<L> BusinessGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(
        logger: Arc<L>,
        cargo: Arc<BusinessCargoGenerator<L>>,
        git_ignore: Arc<BusinessGitIgnoreGenerator<L>>,
        readme: Arc<BusinessReadmeGenerator<L>>,
        lib: Arc<BusinessLibGenerator<L>>,
        entity: Arc<BusinessEntityGenerator<L>>,
        repository: Arc<BusinessRepositoryGenerator<L>>,
        mock: Arc<BusinessMockGenerator<L>>,
        di: Arc<BusinessDiGenerator<L>>,
    ) -> Self {
        Self {
            logger,
            cargo,
            git_ignore,
            readme,
            lib,
            entity,
            repository,
            mock,
            di,
        }
    }

    pub fn generate(&self, params: &BusinessParams) -> Result<(), String> {
        self.cargo.generate(params)?;
        self.git_ignore.generate(params)?;
        self.readme.generate(params)?;
        self.lib.generate(params)?;
        self.entity.generate(params)?;
        self.repository.generate(params)?;
        self.mock.generate(params)?;
        self.di.generate(params)?;

        self.logger.info("Business package is generated.");

        Ok(())
    }
}
