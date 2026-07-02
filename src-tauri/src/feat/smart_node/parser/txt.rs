use crate::feat::smart_node::parser::{ParsedNode, base64::parse_single_uri};
use smartstring::alias::String;
use std::collections::HashMap;

/// 解析纯文本格式
///
/// 每行一个代理 URI:
/// ss://...
/// vmess://...
/// trojan://...
/// vless://...
/// 等等
pub fn parse_txt(content: &str) -> Vec<ParsedNode> {
    let supported_protocols = [
        "ss://",
        "ssr://",
        "vmess://",
        "vless://",
        "trojan://",
        "hysteria2://",
        "hysteria://",
        "tuic://",
        "naive+https://",
        "wireguard://",
    ];

    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| supported_protocols.iter().any(|p| line.starts_with(p)))
        .filter_map(|line| parse_single_uri(line).ok())
        .collect()
}
