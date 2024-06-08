use crate::cli_params::CliParams;
use crate::generator::dto::{
    DtoGenerator, OperationResponseEnumGenerator, ParamsGenerator, PathParamsGenerator,
    RequestGenerator, ResponseGenerator, SchemaGenerator,
};
use crate::generator::server::{
    HandlerGenerator, PreHandlerGenerator, RouterGenerator, ServerGenerator,
};
use crate::generator::{
    CargoTomlGenerator, Combiner, DiGenerator, GitIgnoreGenerator, LibGenerator, OasYamlGenerator,
    ReadmeAdocGenerator,
};
use std::error::Error;
use std::sync::Arc;

pub struct OpenApiGenerator {
    combiner: Arc<Combiner>,
    oas_yaml_generator: Arc<OasYamlGenerator>,
    dto_generator: Arc<DtoGenerator>,
    path_param_generator: Arc<PathParamsGenerator>,
    schema_param_generator: Arc<ParamsGenerator>,
    operation_response_enum_generator: Arc<OperationResponseEnumGenerator>,
    response_generator: Arc<ResponseGenerator>,
    request_generator: Arc<RequestGenerator>,
    handler_generator: Arc<HandlerGenerator>,
    pre_handler_generator: Arc<PreHandlerGenerator>,
    router_generator: Arc<RouterGenerator>,
    server_generator: Arc<ServerGenerator>,
    di_generator: Arc<DiGenerator>,
    lib_generator: Arc<LibGenerator>,
    cargo_toml_generator: Arc<CargoTomlGenerator>,
    readme_adoc_generator: Arc<ReadmeAdocGenerator>,
    gitignore_generator: Arc<GitIgnoreGenerator>,
    schema_generator: Arc<SchemaGenerator>,
}

impl OpenApiGenerator {
    pub fn new(
        combiner: Arc<Combiner>,
        oas_yaml_generator: Arc<OasYamlGenerator>,
        dto_generator: Arc<DtoGenerator>,
        path_param_generator: Arc<PathParamsGenerator>,
        schema_param_generator: Arc<ParamsGenerator>,
        operation_response_enum_generator: Arc<OperationResponseEnumGenerator>,
        response_generator: Arc<ResponseGenerator>,
        request_generator: Arc<RequestGenerator>,
        handler_generator: Arc<HandlerGenerator>,
        pre_handler_generator: Arc<PreHandlerGenerator>,
        router_generator: Arc<RouterGenerator>,
        server_generator: Arc<ServerGenerator>,
        di_generator: Arc<DiGenerator>,
        lib_generator: Arc<LibGenerator>,
        cargo_toml_generator: Arc<CargoTomlGenerator>,
        readme_adoc_generator: Arc<ReadmeAdocGenerator>,
        gitignore_generator: Arc<GitIgnoreGenerator>,
        schema_generator: Arc<SchemaGenerator>,
    ) -> Self {
        Self {
            combiner,
            oas_yaml_generator,
            dto_generator,
            path_param_generator,
            schema_param_generator,
            operation_response_enum_generator,
            response_generator,
            request_generator,
            handler_generator,
            pre_handler_generator,
            router_generator,
            server_generator,
            di_generator,
            lib_generator,
            cargo_toml_generator,
            readme_adoc_generator,
            gitignore_generator,
            schema_generator,
        }
    }

    pub fn generate(&self, cli: CliParams) -> Result<(), Box<dyn Error>> {
        let spec = self.combiner.combine(cli.input.clone());

        let output = cli.output.as_str();

        //generation
        self.oas_yaml_generator.generate(output, &spec)?;
        self.dto_generator.generate(output)?;
        self.path_param_generator.generate(output, &spec)?;
        self.schema_param_generator.generate(output, &spec)?;
        self.operation_response_enum_generator
            .generate(output, &spec)?;
        self.response_generator.generate(output, &spec)?;
        self.request_generator.generate(output, &spec)?;
        self.handler_generator.generate(output, &spec)?;
        self.pre_handler_generator.generate(output, &spec)?;
        self.router_generator.generate(output, &spec)?;
        self.server_generator.generate(output)?;
        self.di_generator.generate(output, &spec)?;
        self.lib_generator.generate(output)?;
        self.cargo_toml_generator.generate(
            output,
            &spec,
            cli.package_name.clone(),
            cli.homepage.clone(),
            cli.repository.clone(),
            cli.documentation.clone(),
        )?;

        self.readme_adoc_generator.generate(
            output,
            &spec,
            cli.package_name,
            cli.homepage,
            cli.repository,
            cli.documentation,
        )?;
        self.gitignore_generator.generate(output)?;
        self.schema_generator.generate(output, &spec)?;

        Ok(())
    }
}
