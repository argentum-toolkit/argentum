use handlebars::Handlebars;
use serde::Serialize;
use std::fs::File;
use std::sync::Arc;

pub struct Renderer {
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

#[cfg(test)]

mod tests {
    use super::*;
    use handlebars::Handlebars;
    use serde::Serialize;
    use std::sync::Arc;

    #[derive(Serialize)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    fn test_render_to_result() {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("test_template", "Hello, {{name}}! Your value is {{value}}.")
            .unwrap();

        let renderer = Renderer::new(Arc::new(handlebars));
        let data = TestData {
            name: "World".to_string(),
            value: 42,
        };

        let result = renderer.render_to_result("test_template", data).unwrap();
        assert_eq!(result, "Hello, World! Your value is 42.");
    }

    #[test]
    fn test_render_to_file() {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("test_template", "Hello, {{name}}! Your value is {{value}}.")
            .unwrap();

        let renderer = Renderer::new(Arc::new(handlebars));
        let data = TestData {
            name: "World".to_string(),
            value: 42,
        };

        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("output.txt");

        renderer
            .render(
                temp_dir.path().to_str().unwrap(),
                "test_template",
                data,
                "/output.txt",
            )
            .unwrap();

        let content = std::fs::read_to_string(output_path).unwrap();
        assert_eq!(content, "Hello, World! Your value is 42.");
    }

    #[test]
    fn test_render_to_file_with_nested_path() {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("test_template", "Hello, {{name}}! Your value is {{value}}.")
            .unwrap();

        let renderer = Renderer::new(Arc::new(handlebars));
        let data = TestData {
            name: "World".to_string(),
            value: 42,
        };

        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("nested/output.txt");

        renderer
            .render(
                temp_dir.path().to_str().unwrap(),
                "test_template",
                data,
                "/nested/output.txt",
            )
            .unwrap();

        let content = std::fs::read_to_string(output_path).unwrap();
        assert_eq!(content, "Hello, World! Your value is 42.");
    }
}
