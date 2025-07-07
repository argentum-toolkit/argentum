mod form_data_generator;
mod generator;
pub mod input;
mod input_generator;
mod web_boilerplate_generator;

pub(crate) use form_data_generator::FormDataGenerator;
pub(crate) use generator::UiGenerator;
pub(crate) use input_generator::InputGenerator;
pub(crate) use web_boilerplate_generator::WebBoilerplateGenerator;
