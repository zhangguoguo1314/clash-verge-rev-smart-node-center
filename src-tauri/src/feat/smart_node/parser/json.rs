use crate::feat::smart_node::parser::ParsedNode;
use smartstring::alias::String;
use std::collections::HashMap;

/// 解析 JSON 格式的代理配置
///
/// 支持两种格式:
/// - Clash 导出格式: {"proxies": [...]}
/// - 直接数组格式: [...]
pub fn parse_json(content: &str) -> Result<Vec<ParsedNode>, String> {
    let value: serde_json::Value =
        serde_json::from_str(content).map_err(|e| format!("json parse error: {}", e))?;

    let proxies = match &value {
        serde_json::Value::Object(map) => {
            map.get("proxies")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "no 'proxies' array found in JSON object".to_string())?
        }
        serde_json::Value::Array(arr) => arr,
        _ => return Err("invalid JSON structure, expected object or array".to_string()),
    };

    let mut nodes = Vec::new();

    for proxy in proxies {
        if let Some(node) = parse_json_proxy(proxy) {
            nodes.push(node);
        }
    }

    Ok(nodes)
}

fn parse_json_proxy(proxy: &serde_json::Value) -> Option<ParsedNode> {
    let name = proxy.get("name")?.as_str()?.to_string();
    let protocol = proxy.get("type")?.as_str()?.to_string();
    let address = proxy.get("server")?.as_str()?.to_string();
    let port = proxy.get("port")?.as_u64()? as u16;

    let uri = format!("{}://{}@{}:{}", protocol, "[encoded]", address, port);

    Some(ParsedNode {
        name,
        protocol,
        address,
        port,
        uri,
        extra: HashMap::new(),
    })
}
