use base64::Engine as _;
use base64::engine::general_purpose;
use crate::feat::smart_node::parser::ParsedNode;
use smartstring::alias::String;
use std::collections::HashMap;

/// Base64 解码后逐行解析 URI
///
/// 支持:
/// - 标准 Base64
/// - URL-safe Base64
/// - 自动补齐 padding
pub fn parse_base64(content: &str) -> Result<Vec<ParsedNode>, String> {
    let cleaned: String = content.chars().filter(|c| !c.is_whitespace()).collect();

    // 先尝试标准 Base64
    let decoded = match try_decode_standard(&cleaned) {
        Ok(d) => d,
        Err(_) => {
            // 尝试 URL-safe Base64
            try_decode_url_safe(&cleaned)?
        }
    };

    let text = String::from_utf8(decoded)
        .map_err(|e| format!("base64 decode result is not utf8: {}", e))?;
    parse_uri_lines(&text)
}

fn try_decode_standard(input: &str) -> Result<Vec<u8>, String> {
    general_purpose::STANDARD
        .decode(input)
        .map_err(|e| format!("base64 standard decode error: {}", e))
}

fn try_decode_url_safe(input: &str) -> Result<Vec<u8>, String> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(input)
        .map_err(|e| format!("base64 url-safe decode error: {}", e))
}

/// 逐行解析 URI
fn parse_uri_lines(text: &str) -> Vec<ParsedNode> {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && is_supported_protocol(line))
        .filter_map(|line| parse_single_uri(line).ok())
        .collect()
}

/// 判断是否为支持的协议 URI
fn is_supported_protocol(uri: &str) -> bool {
    let protocols = [
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
    protocols.iter().any(|p| uri.starts_with(p))
}

/// 解析单个代理 URI
fn parse_single_uri(uri: &str) -> Result<ParsedNode, String> {
    let protocol = extract_protocol(uri)?;
    let (name, address, port) = parse_proxy_uri(uri, &protocol)?;

    Ok(ParsedNode {
        name,
        protocol,
        address,
        port,
        uri: uri.to_string(),
        extra: HashMap::new(),
    })
}

fn extract_protocol(uri: &str) -> Result<String, String> {
    if let Some(pos) = uri.find("://") {
        Ok(uri[..pos].to_string())
    } else {
        Err(format!("invalid URI format: {}", uri))
    }
}

fn parse_proxy_uri(uri: &str, protocol: &str) -> Result<(String, String, u16), String> {
    let mut name = String::from("unnoded");
    let mut address = String::new();
    let mut port: u16 = 0;

    let after_scheme = uri
        .strip_prefix(&format!("{}://", protocol))
        .unwrap_or(uri);

    // 提取 fragment (节点名称)
    let (main_part, fragment) = if let Some(frag_pos) = after_scheme.rfind('#') {
        (&after_scheme[..frag_pos], Some(&after_scheme[frag_pos + 1..]))
    } else {
        (after_scheme, None)
    };

    if let Some(frag) = fragment {
        if !frag.is_empty() {
            name = urldecode(frag);
        }
    }

    // 提取 query params
    let body = if let Some(q_pos) = main_part.find('?') {
        &main_part[..q_pos]
    } else {
        main_part
    };

    // 根据协议解析地址和端口
    match protocol {
        "ss" => {
            if let Some(at_pos) = body.rfind('@') {
                let host_port = &body[at_pos + 1..];
                (address, port) = parse_host_port(host_port)?;
            }
        }
        "vmess" | "vless" | "trojan" | "hysteria2" | "hysteria" | "tuic" => {
            if let Some(at_pos) = body.rfind('@') {
                let host_port = &body[at_pos + 1..];
                (address, port) = parse_host_port(host_port)?;
            } else {
                return Err(format!("complex {} URI parsing requires @ separator", protocol));
            }
        }
        "ssr" => {
            if let Some(at_pos) = body.rfind('@') {
                let host_port = &body[at_pos + 1..];
                (address, port) = parse_host_port(host_port)?;
            }
        }
        _ => {
            (address, port) = parse_host_port(body)?;
        }
    }

    Ok((name, address, port))
}

fn parse_host_port(host_port: &str) -> Result<(String, u16), String> {
    // 处理 [ipv6]:port 格式
    if host_port.starts_with('[') {
        if let Some(bracket_end) = host_port.find(']') {
            let ipv6 = &host_port[1..bracket_end];
            let rest = &host_port[bracket_end + 1..];
            let port_str = rest.strip_prefix(':').unwrap_or(rest);
            let port: u16 = port_str.parse().map_err(|_| "invalid port")?;
            return Ok((ipv6.to_string(), port));
        }
    }

    // host:port
    if let Some(colon_pos) = host_port.rfind(':') {
        let host = &host_port[..colon_pos];
        let port_str = &host_port[colon_pos + 1..];
        let port: u16 = port_str.parse().map_err(|_| "invalid port")?;
        return Ok((host.to_string(), port));
    }

    Err(format!("invalid host:port format: {}", host_port))
}

fn urldecode(s: &str) -> String {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(
                std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""),
                16,
            ) {
                result.push(byte);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(result).unwrap_or_else(|_| s.to_string())
}
