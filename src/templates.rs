use log::{debug, error};
use std::collections::HashMap;
use std::fs;

/// Simple template engine for HTML templates
pub struct TemplateEngine {
    templates: HashMap<String, String>,
}

impl TemplateEngine {
    /// Create a new template engine and load all templates
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut templates = HashMap::new();

        // Load all template files
        let template_files = [
            "base.html",
            "homepage.html",
            "blog_list.html",
            "blog_post.html",
            "404.html",
            "components/fab.html",
            "components/footer.html",
        ];

        for template_name in template_files.iter() {
            let template_path = format!("templates/{template_name}");
            match fs::read_to_string(&template_path) {
                Ok(content) => {
                    templates.insert(template_name.to_string(), content);
                    debug!("Loaded template: {template_name}");
                }
                Err(e) => {
                    error!("Failed to load template {template_name}: {e}");
                    return Err(format!("Failed to load template {template_name}: {e}").into());
                }
            }
        }

        Ok(Self { templates })
    }

    /// Render a template with variables
    pub fn render(
        &self,
        template_name: &str,
        variables: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let template = self
            .templates
            .get(template_name)
            .ok_or_else(|| format!("Template '{template_name}' not found"))?;

        let mut result = template.clone();

        // Replace variables in the format {{variable_name}}
        for (key, value) in variables {
            let placeholder = format!("{{{{{key}}}}}");
            result = result.replace(&placeholder, value);
        }

        // Handle component includes like {{fab}} and {{footer}}
        result = self.process_includes(&result, variables)?;

        Ok(result)
    }

    /// Process component includes in templates
    fn process_includes(
        &self,
        content: &str,
        _variables: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut result = content.to_string();

        // Replace {{fab}} with FAB component
        if result.contains("{{fab}}") {
            let fab_vars = {
                let mut vars = HashMap::new();
                vars.insert("homepage_anchor".to_string(), "#".to_string());
                vars
            };
            let fab_content = self.render_component("fab", &fab_vars)?;
            result = result.replace("{{fab}}", &fab_content);
        }

        // Replace {{footer}} with footer component
        if result.contains("{{footer}}") {
            let footer_content = self.render_component("footer", &HashMap::new())?;
            result = result.replace("{{footer}}", &footer_content);
        }

        Ok(result)
    }

    /// Render a component template
    pub fn render_component(
        &self,
        component_name: &str,
        variables: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let template_name = format!("components/{component_name}.html");
        self.render(&template_name, variables)
    }

    /// Render the base template with content and meta description
    pub fn render_base_with_meta(
        &self,
        title: &str,
        content: &str,
        meta_description: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.render_base_with_meta_and_css(title, content, meta_description, None)
    }

    /// Render the base template with content, meta description, and optional additional CSS
    pub fn render_base_with_meta_and_css(
        &self,
        title: &str,
        content: &str,
        meta_description: &str,
        additional_css: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut variables = HashMap::new();
        variables.insert("title".to_string(), title.to_string());
        variables.insert("content".to_string(), content.to_string());
        variables.insert("meta_description".to_string(), meta_description.to_string());

        // Add additional CSS if provided
        let css_link = additional_css
            .map(|css| format!("<link rel=\"stylesheet\" href=\"{css}\">"))
            .unwrap_or_default();
        variables.insert("additional_css".to_string(), css_link);

        self.render("base.html", &variables)
    }

    /// Render the base template with content, meta description, and multiple additional CSS files
    pub fn render_base_with_meta_and_css_list(
        &self,
        title: &str,
        content: &str,
        meta_description: &str,
        additional_css: &[&str],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut variables = HashMap::new();
        variables.insert("title".to_string(), title.to_string());
        variables.insert("content".to_string(), content.to_string());
        variables.insert("meta_description".to_string(), meta_description.to_string());

        let css_links = if additional_css.is_empty() {
            String::new()
        } else {
            additional_css
                .iter()
                .map(|css| format!("<link rel=\"stylesheet\" href=\"{}\">", css))
                .collect::<Vec<_>>()
                .join("\n")
        };
        variables.insert("additional_css".to_string(), css_links);

        self.render("base.html", &variables)
    }
}
