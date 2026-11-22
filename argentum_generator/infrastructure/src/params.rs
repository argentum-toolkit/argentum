#[derive(Debug, Clone)]
pub struct ApiDocParams {
    pub component_name: String,
}

#[derive(Debug, Clone)]
pub struct BusinessParams {
    pub component_name: String,
    pub entity_name: String,
}

#[derive(Debug, Clone)]
pub struct DbInfrastructureParams {
    pub entity_name: String,
    pub component_name: String,
}

#[derive(Debug, Clone)]
pub struct InfrastructureParams {
    pub component_name: String,
    pub entity_name: String,
}

#[derive(Debug, Clone)]
pub struct ComponentParams {
    pub component_name: String,
    pub entity_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DevopsParams {
    pub component_name: String,
}

#[derive(Debug, Clone)]
pub struct FeAppParams {
    pub app_name: String,
}

#[derive(Debug, Clone)]
pub struct UiParams {
    pub component_name: String,
}
