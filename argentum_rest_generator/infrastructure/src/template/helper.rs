use handlebars::handlebars_helper;

handlebars_helper!(package_name_to_container_provider_name: |package_name: Option<String>| {
    let package_name = package_name.unwrap_or_default();
    let component_name = package_name
        .strip_prefix("argentum_")
        .unwrap_or(&package_name);
    let component_name = component_name
        .strip_suffix("_rest")
        .unwrap_or(component_name);

    component_name.to_owned() + "_rest_provide"
});
