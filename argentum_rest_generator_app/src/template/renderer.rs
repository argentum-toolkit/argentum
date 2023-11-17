use handlebars::Handlebars;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::sync::Arc;

pub(crate) struct Renderer {
    handlebars: Arc<Handlebars<'static>>,
}

impl Renderer {
    pub fn new(handlebars: Arc<Handlebars<'static>>) -> Self {
        Self { handlebars }
    }

    pub fn render<T>(
        &self,
        base_output_path: &str,
        template_name: &str,
        data: T,
        output_path: &str,
    ) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
    {
        let file_path = base_output_path.to_owned() + output_path;

        let path = std::path::Path::new(file_path.as_str());
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        let mut output_file = File::create(file_path)?;

        self.handlebars
            .render_to_write(template_name, &data, &mut output_file)?;

        Ok(())
    }
}
