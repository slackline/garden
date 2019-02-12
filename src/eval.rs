extern crate shellexpand;
extern crate dirs;

use super::model;


/// Expand variables across all scopes (garden, tree, and global)
fn expand_tree_vars(
    config: &mut model::Configuration,
    tree_idx: model::TreeIndex,
    garden_idx: Option<model::GardenIndex>,
    name: &str,
) -> Result<Option<String>, String> {

    let mut var_idx: usize = 0;
    let mut found = false;

    // First check for the variable at the garden scope.
    // Garden scope overrides tree and global scope.
    if garden_idx.is_some() {
        for var in &config.gardens[garden_idx.unwrap()].variables {
            if var.name == name {
                if var.value.is_some() {
                    return Ok(Some(var.value.as_ref().unwrap().to_string()));
                }
                found = true;
                break;
            }
            var_idx += 1;
        }

        if found {
            let expr =
                config
                .gardens[garden_idx.unwrap()]
                .variables[var_idx]
                .expr.to_string();

            let result = tree_value(config, expr, tree_idx, garden_idx);
            config
                .gardens[garden_idx.unwrap()]
                .variables[var_idx]
                .value = Some(result.to_string());
            return Ok(Some(result.to_string()));
        }
    }

    // Nothing was found -- check for the variable in tree scope.
    found = false;
    var_idx = 0;

    for var in &config.trees[tree_idx].variables {
        if var.name == name {
            if var.value.is_some() {
                return Ok(Some(var.value.as_ref().unwrap().to_string()));
            }
            found = true;
            break;
        }
        var_idx += 1;
    }

    if found {
        let expr = config.trees[tree_idx].variables[var_idx].expr.to_string();
        let result = tree_value(config, expr, tree_idx, garden_idx);
        config
            .trees[tree_idx]
            .variables[var_idx]
            .value = Some(result.to_string());
        return Ok(Some(result));
    }

    // Nothing was found.  Check for the variable in global/config scope.
    found = false;
    var_idx = 0;

    for var in &mut config.variables {
        if var.name == name {
            if var.value.is_some() {
                return Ok(Some(var.value.as_ref().unwrap().to_string()));
            }
            found = true;
            break;
        }
        var_idx += 1;
    }

    if found {
        let expr = config.variables[var_idx].expr.to_string();
        let result = tree_value(config, expr, tree_idx, garden_idx);
        config.variables[var_idx].value = Some(result.to_string());

        return Ok(Some(result));
    }

    // Nothing was found -> empty value
    return Ok(Some("".to_string()));
}


/// Expand variables at global scope only
fn expand_vars(
    config: &mut model::Configuration,
    name: &str,
) -> Result<Option<String>, String> {

    let mut var_idx: usize = 0;
    let mut found = false;

    for var in &mut config.variables {
        if var.name == name {
            if var.value.is_some() {
                return Ok(Some(var.value.as_ref().unwrap().to_string()));
            }
            found = true;
            break;
        }
        var_idx += 1;
    }

    if found {
        let expr = config.variables[var_idx].expr.to_string();
        let result = value(config, expr);
        config.variables[var_idx].value = Some(result.to_string());

        return Ok(Some(result));
    }

    // Nothing was found -> empty value
    return Ok(Some("".to_string()));
}


/// Resolve a variable in a garden/tree/global scope
pub fn tree_value<S: Into<String>>(
    config: &mut model::Configuration,
    expr: S,
    tree_idx: model::TreeIndex,
    garden_idx: Option<model::GardenIndex>,
) -> String {
    let expr_str: String = expr.into();

    let expanded = shellexpand::full_with_context(
        &expr_str, dirs::home_dir,
        |x| { return expand_tree_vars(config, tree_idx, garden_idx, x); }
        ).unwrap().to_string();

    return expanded;
}


/// Resolve a variable in configuration/global scope
pub fn value<S: Into<String>>(
    config: &mut model::Configuration,
    expr: S,
) -> String {
    let expr_str: String = expr.into();

    let expanded = shellexpand::full_with_context(
        &expr_str, dirs::home_dir,
        |x| { return expand_vars(config, x); }
        ).unwrap().to_string();

    return expanded;
}