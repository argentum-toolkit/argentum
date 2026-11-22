#[derive(Debug, Clone)]
pub struct ApiDocParams {
    pub project_prefix: String,
    pub component_name: String,
}

#[derive(Debug, Clone)]
pub struct BusinessParams {
    pub project_prefix: String,
    pub component_name: String,
    pub entity_name: String,
}

#[derive(Debug, Clone)]
pub struct DbInfrastructureParams {
    pub entity_name: String,
    pub project_prefix: String,
    pub component_name: String,
}

#[derive(Debug, Clone)]
pub struct InfrastructureParams {
    pub project_prefix: String,
    pub component_name: String,
    pub entity_name: String,
}

#[derive(Debug, Clone)]
pub struct ComponentParams {
    pub project_prefix: String,
    pub component_name: String,
    pub entity_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DevopsParams {
    pub project_prefix: String,
    pub component_name: String,
}

#[derive(Debug, Clone)]
pub struct FeAppParams {
    pub app_name: String,
}

#[derive(Debug, Clone)]
pub struct UiParams {
    pub project_prefix: String,
    pub component_name: String,
}

#[derive(Debug, Clone)]
pub struct RestParams {
    pub project_prefix: String,
    pub component_name: String,
}
