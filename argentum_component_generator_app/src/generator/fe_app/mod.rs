mod fe_app_generator;
pub(crate) use fe_app_generator::FeAppGenerator;

mod fe_cargo_generator;
pub(crate) use fe_cargo_generator::FeCargoGenerator;

mod fe_style_generator;
pub(crate) use fe_style_generator::FeStyleGenerator;

mod fe_gitignore_generator;
pub(crate) use fe_gitignore_generator::FeGitIgnoreGenerator;

mod fe_readme_adoc_generator;
pub(crate) use fe_readme_adoc_generator::FeReadmeAdocGenerator;

mod fe_rsx_generator;
pub(crate) use fe_rsx_generator::FeRsxGenerator;

mod fe_main_generator;
pub(crate) use fe_main_generator::FeMainGenerator;

mod fe_route_generator;
pub(crate) use fe_route_generator::FeRouteGenerator;
