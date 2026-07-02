use crate::feat::smart_node::parser::ParsedNode;
use smartstring::alias::String;
use std::collections::HashMap;

/// 解析 YAML 格式的代理配置
///
/// 支持 Clash 配置格式: proxies 列表
/// 每个代理项包含: name, type, server, port 等字段
pub fn parse_yaml(content: &str) -> Result<Vec<ParsedNode>, String> {
    let value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(content).map_err(|e| format!("yaml parse error: {}", e))?;

    let proxies = extract_proxies(&value)?;

    let mut nodes = Vec::new();

    for proxy in &proxies {
        let node = parse_yaml_proxy(proxy)?;
        nodes.push(node);
    }

    Ok(nodes)
}

fn extract_proxies(value: &serde_yaml_ng::Value) -> Result<Vec<&serde_yaml_ng::Mapping>, String> {
    match value {
        serde_yaml_ng::Value::Mapping(map) => {
            if let Some(serde_yaml_ng::Value::Sequence(seq)) = map.get("proxies") {
                Ok(seq.iter().filter_map(|v| v.as_mapping()).collect())
            } else {
                Err("no 'proxies' key found in YAML".to_string())
            }
        }
        serde_yaml_ng::Value::Sequence(seq) => {
            Ok(seq.iter().filter_map(|v| v.as_mapping()).collect())
        }
        _ => Err("invalid YAML structure, expected mapping or sequence".to_string()),
    }
}

fn parse_yaml_proxy(proxy: &serde_yaml_ng::Mapping) -> Result<ParsedNode, String> {
    let name = get_string_field(proxy, "name").unwrap_or_else(|| "unnamed".into());
    let protocol = get_string_field(proxy, "type")
        .ok_or_else(|| "missing 'type' field".to_string())?;
    let address = get_string_field(proxy, "server")
        .ok_or_else(|| "missing 'server' field".to_string())?;
    let port = get_u16_field(proxy, "port")
        .ok_or_else(|| "missing 'port' field".to_string())?;

    // 构建 URI (简化)
    let uri = format!("{}://{}@{}:{}", protocol, "[encoded]", address, port);

    Ok(ParsedNode {
        name,
        protocol,
        address,
        port,
        uri,
        extra: HashMap::new(),
    })
}

fn get_string_field(map: &serde_yaml_ng::Mapping, key: &str) -> Option<String> {
    map.get(&serde_yaml_ng::Value::String(key.into()))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn get_u16_field(map: &serde_yaml_ng::Mapping, key: &str) -> Option<u16> {
    map.get(&serde_yaml_ng::Value::String(key.into()))
        .and_then(|v| v.as_u64())
        .map(|n| n as u16)
}
