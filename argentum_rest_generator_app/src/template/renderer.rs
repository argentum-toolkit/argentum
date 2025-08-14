use handlebars::Handlebars;
use serde::Serialize;
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
    ) -> Result<(), String>
    where
        T: Serialize,
    {
        let file_path = base_output_path.to_owned() + output_path;

        let path = std::path::Path::new(file_path.as_str());
        let prefix = path.parent().ok_or("Can't read parent path for file")?;
        std::fs::create_dir_all(prefix).map_err(|e| format!("Can't create dir. Error: {e}"))?;

        let mut output_file = File::create(&file_path)
            .map_err(|e| format!("Can't create file `{file_path}`. Error: {e}"))?;

        let _ = self
            .handlebars
            .render_to_write(template_name, &data, &mut output_file);

        Ok(())
    }

    pub fn render_to_result<T>(&self, template_name: &str, data: T) -> Result<String, String>
    where
        T: Serialize,
    {
        let res = self
            .handlebars
            .render(template_name, &data)
            .map_err(|e| format!("Can't render template `{template_name}`. Error: {e}"))?;

        Ok(res)
    }
}
