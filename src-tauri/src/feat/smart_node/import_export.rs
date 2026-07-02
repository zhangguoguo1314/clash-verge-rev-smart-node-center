use crate::config::smart_node::ImportResult;
use crate::feat::smart_node::parser::{
    ParsedNode,
    base64::parse_base64,
    csv::parse_csv,
    json::parse_json,
    txt::parse_txt,
    yaml::parse_yaml,
};
use clash_verge_draft::Draft;
use clash_verge_logging::{Type, logging};

/// 从内容导入节点
///
/// 支持多种格式: base64, yaml, json, txt, csv
pub async fn import_from_content(
    content: &str,
    format: &str,
    source_name: &str,
) -> Result<ImportResult, String> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Ok(ImportResult::default());
    }

    let parsed = match format.to_lowercase().as_str() {
        "base64" => parse_base64(trimmed).map_err(|e| e.to_string())?,
        "yaml" | "yml" => parse_yaml(trimmed).map_err(|e| e.to_string())?,
        "json" => parse_json(trimmed).map_err(|e| e.to_string())?,
        "txt" | "text" => parse_txt(trimmed),
        "csv" => parse_csv(trimmed).map_err(|e| e.to_string())?,
        _ => return Err(format!("unsupported format: {}", format)),
    };

    logging!(
        info,
        Type::Core,
        "SNC: parsed {} nodes from {} format",
        parsed.len(),
        format
    );

    // 转换为 SmartNodeInput 并添加到 pool
    let inputs: Vec<crate::feat::smart_node::pool::SmartNodeInput> = parsed
        .into_iter()
        .map(|p| crate::feat::smart_node::pool::SmartNodeInput {
            name: p.name,
            protocol: p.protocol,
            address: p.address,
            port: p.port,
            uri: Some(p.uri),
            source: Some(source_name.to_string()),
            source_url: None,
            tags: Vec::new(),
        })
        .collect();

    let pool = crate::config::smart_node::ISmartNodePool::load_file().await;
    let draft = Draft::new(pool);
    let result = crate::feat::smart_node::pool::add_nodes(inputs, &draft).await;

    Ok(result)
}

/// 导出节点
///
/// 支持多种输出格式: uri, yaml, json, csv
pub fn export_nodes(
    node_uids: &[String],
    pool: &Draft<crate::config::smart_node::ISmartNodePool>,
    format: &str,
) -> Result<String, String> {
    let data = pool.data_arc();
    let nodes: Vec<_> = data
        .nodes
        .iter()
        .filter(|n| node_uids.contains(&n.uid))
        .cloned()
        .collect();

    if nodes.is_empty() {
        return Ok(String::new());
    }

    let output = match format.to_lowercase().as_str() {
        "uri" | "txt" => export_as_uri(&nodes),
        "yaml" | "yml" => export_as_yaml(&nodes).map_err(|e| e.to_string())?,
        "json" => export_as_json(&nodes).map_err(|e| e.to_string())?,
        "csv" => export_as_csv(&nodes),
        _ => return Err(format!("unsupported export format: {}", format)),
    };

    Ok(output)
}

fn export_as_uri(nodes: &[crate::config::smart_node::SmartNode]) -> String {
    nodes
        .iter()
        .filter_map(|n| n.uri.as_ref())
        .map(|uri| uri.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn export_as_yaml(
    nodes: &[crate::config::smart_node::SmartNode],
) -> Result<String, serde_yaml_ng::Error> {
    let proxy_list: Vec<serde_yaml_ng::Value> = nodes
        .iter()
        .map(|n| {
            serde_yaml_ng::Value::Mapping({
                let mut map = serde_yaml_ng::Mapping::new();
                map.insert(
                    serde_yaml_ng::Value::String("name".into()),
                    serde_yaml_ng::Value::String(n.name.clone()),
                );
                map.insert(
                    serde_yaml_ng::Value::String("type".into()),
                    serde_yaml_ng::Value::String(n.protocol.clone()),
                );
                map.insert(
                    serde_yaml_ng::Value::String("server".into()),
                    serde_yaml_ng::Value::String(n.address.clone()),
                );
                map.insert(
                    serde_yaml_ng::Value::String("port".into()),
                    serde_yaml_ng::Value::Number(serde_yaml_ng::Number::from(n.port)),
                );
                if let Some(uri) = &n.uri {
                    map.insert(
                        serde_yaml_ng::Value::String("uri".into()),
                        serde_yaml_ng::Value::String(uri.clone()),
                    );
                }
                map
            })
        })
        .collect();

    let proxies_map = serde_yaml_ng::Value::Sequence(proxy_list);
    serde_yaml_ng::to_string(&proxies_map)
}

fn export_as_json(
    nodes: &[crate::config::smart_node::SmartNode],
) -> Result<String, serde_json::Error> {
    let proxy_list: Vec<serde_json::Value> = nodes
        .iter()
        .map(|n| {
            serde_json::json!({
                "name": n.name,
                "type": n.protocol,
                "server": n.address,
                "port": n.port,
            })
        })
        .collect();

    serde_json::to_string_pretty(&serde_json::json!({ "proxies": proxy_list }))
}

fn export_as_csv(nodes: &[crate::config::smart_node::SmartNode]) -> String {
    let mut lines = Vec::new();
    lines.push("name,protocol,address,port,uri".to_string());
    for n in nodes {
        let uri = n.uri.as_deref().unwrap_or("");
        lines.push(format!("{},{},{},{},{}", n.name, n.protocol, n.address, n.port, uri));
    }
    lines.join("\n")
}
