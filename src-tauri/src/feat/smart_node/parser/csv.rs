use crate::feat::smart_node::parser::ParsedNode;
use smartstring::alias::String;
use std::collections::HashMap;

/// 解析 CSV 格式的代理配置
///
/// 期望表头行包含: name, protocol, address, port
/// 后续行为数据行
pub fn parse_csv(content: &str) -> Result<Vec<ParsedNode>, String> {
    let mut lines = content.lines();

    // 跳过表头行
    let header = lines.next().unwrap_or("");
    let headers: Vec<&str> = header.split(',').map(|s| s.trim()).collect();

    // 查找各列索引
    let name_idx = find_column(&headers, "name");
    let protocol_idx = find_column(&headers, "protocol").or(find_column(&headers, "type"));
    let address_idx = find_column(&headers, "address").or(find_column(&headers, "server"));
    let port_idx = find_column(&headers, "port");

    let name_idx = name_idx.ok_or_else(|| "CSV missing 'name' column".to_string())?;
    let protocol_idx = protocol_idx.ok_or_else(|| "CSV missing 'protocol' column".to_string())?;
    let address_idx = address_idx.ok_or_else(|| "CSV missing 'address' column".to_string())?;
    let port_idx = port_idx.ok_or_else(|| "CSV missing 'port' column".to_string())?;

    let mut nodes = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let fields: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();

        if fields.len() <= name_idx.max(protocol_idx).max(address_idx).max(port_idx) {
            continue;
        }

        let name = fields[name_idx].to_string();
        let protocol = fields[protocol_idx].to_string();
        let address = fields[address_idx].to_string();
        let port: u16 = fields[port_idx]
            .parse()
            .unwrap_or(0);

        if port == 0 || address.is_empty() {
            continue;
        }

        let uri = format!("{}://{}@{}:{}", protocol, "[encoded]", address, port);

        nodes.push(ParsedNode {
            name,
            protocol,
            address,
            port,
            uri,
            extra: HashMap::new(),
        });
    }

    Ok(nodes)
}

fn find_column(headers: &[&str], target: &str) -> Option<usize> {
    headers
        .iter()
        .position(|&h| h.eq_ignore_ascii_case(target))
}
