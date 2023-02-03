use std::collections::HashMap;

use base64::{engine::general_purpose, Engine};

use crate::config::{read, AppConfig};

use super::error::{GenerateLinkError, ParseLinkError};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct ConfigJson {
    log: LogObject,
    api: ApiObject,
    dns: DnsObject,
    inbounds: Vec<InboundObject>,
    outbound: Vec<OutboundObject>,
    policy: PolicyObject,
    routing: RoutingObject,
    stats: StatsObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct LogObject {
    #[serde(alias = "loglevel")]
    log_level: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct ApiObject {
    tag: String,
    services: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct DnsObject {
    server: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct InboundObject {
    port: i32,
    listen: String,
    protocol: String,
    settings: InboundConfigurationObject,
    tag: String,
    sniffing: SniffingObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
enum InboundConfigurationObject {
    Socks {
        auth: String,
        udp: bool,
        ip: String,
        user_level: i32,
    },
    Http {
        timeout: Option<i32>,
        accounts: Option<HttpUserObject>,
        allow_transparent: bool,
        level: Option<i32>,
    },
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct HttpUserObject {
    user: String,
    pass: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct OutboundObject {
    #[serde(rename = "sendThrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    send_through: Option<String>,
    protocol: String,
    settings: OutboundConfigurationObject,
    tag: String,

    stream_settings: StreamSettingsObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_settings: Option<ProxySettingsObject>,
    mux: MuxObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct MuxObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency: Option<i32>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct ProxySettingsObject {
    tag: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
enum OutboundConfigurationObject {
    #[serde(rename = "vnext")]
    Vmess { vnext: Vec<VmessServerObject> },
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct StreamSettingsObject {
    #[serde(default = "default_network")]
    network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_settings: Option<TcpObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kcp_settings: Option<KcpObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ws_settings: Option<WebSocketObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_settings: Option<HttpObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ds_settings: Option<DomainSocketObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quic_settings: Option<QUICObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sockopt: Option<SockoptObject>,
}

fn default_network() -> String {
    "tcp".to_string()
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct QUICObject {
    security: String,
    key: String,
    header: Option<QUICHeaderObject>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct QUICHeaderObject {
    #[serde(rename = "type")]
    quic_type: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct DomainSocketObject {
    path: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct HttpObject {
    host: Vec<String>,
    path: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct WebSocketObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct KcpObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    mtu: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tti: Option<i32>,
    uplink_capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    downlink_capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    congestion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_buffer_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_buffer_size: Option<i32>,
    header: KcpHeaderObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct KcpHeaderObject {
    #[serde(rename = "type")]
    kcp_type: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct TcpObject {
    header: TcpHeaderObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
enum TcpHeaderObject {
    NoneHeaderObject {
        #[serde(rename = "type")]
        tcp_type: String,
    },
    HttpHeaderObject {
        #[serde(rename = "type")]
        tcp_type: String,
        request: Option<HTTPRequestObject>,
        response: Option<HTTPResponseObject>,
    },
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct HTTPRequestObject {
    version: String,
    method: String,
    path: Vec<String>,
    headers: HashMap<String, Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct HTTPResponseObject {
    version: String,
    status: String,
    reason: String,
    headers: HashMap<String, Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SockoptObject {
    mark: i32,
    tcp_fast_open: bool,
    tproxy: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct VmessServerObject {
    address: String,
    port: i32,
    users: Vec<UserObject>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct UserObject {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    alter_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<i32>,
    security: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SniffingObject {
    #[serde(alias = "destOverride")]
    dest_override: Vec<String>,
    enabled: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct PolicyObject {
    system: SystemPolicyObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SystemPolicyObject {
    #[serde(alias = "statsInboundUplink")]
    stats_inbound_uplink: bool,
    #[serde(alias = "statsInboundDownlink")]
    stats_inbound_downlink: bool,
    stats_outbound_uplink: bool,
    stats_outbound_downlink: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct RoutingObject {
    #[serde(alias = "domainStrategy")]
    domain_strategy: String,
    #[serde(alias = "domainMatcher")]
    domain_matcher: String,
    rules: Vec<RuleObject>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct RuleObject {
    #[serde(rename = "type")]
    rule_type: String,
    #[serde(alias = "inboundTag")]
    inbound_tag: Vec<String>,
    #[serde(alias = "outboundTag")]
    outbound_tag: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Outbounds {
    outbounds: Vec<OutboundObject>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct StatsObject {}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Base64LinkObject {
    pub v: String,
    pub ps: String,
    pub add: String,
    pub port: i32,
    pub id: String,
    pub aid: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scy: Option<String>,
    pub net: String,
    #[serde(rename = "type")]
    pub base64_type: String,
    pub host: Option<String>,
    pub path: Option<String>,
    pub tls: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
}

fn get_inbound_object(config: &AppConfig) -> Vec<InboundObject> {
    let mut result = Vec::new();
    if config.sock5_status {
        result.push(InboundObject {
            port: config.socks_port,
            listen: config.socks_bind.clone(),
            protocol: "socks".to_string(),
            settings: InboundConfigurationObject::Socks {
                auth: "noauth".to_string(),
                udp: true,
                ip: "127.0.0.1".to_string(),
                user_level: 0,
            },
            tag: "SOCK5_IN".to_string(),
            sniffing: SniffingObject {
                dest_override: vec!["http".to_string(), "tls".to_string(), "fakedns".to_string()],
                enabled: true,
            },
        });
    }
    if config.http_status {
        result.push(InboundObject {
            port: config.http_port,
            listen: config.http_bind.clone(),
            protocol: "http".to_owned(),
            settings: InboundConfigurationObject::Http {
                timeout: Some(0),
                accounts: None,
                allow_transparent: true,
                level: None,
            },
            tag: "HTTP_IN".to_owned(),
            sniffing: SniffingObject {
                dest_override: vec!["http".to_string(), "tls".to_string(), "fakedns".to_string()],
                enabled: true,
            },
        });
    }
    result
}

pub fn generate(outbound: &Outbounds) -> String {
    let config = read();
    let bind = outbound.clone();
    let config = ConfigJson {
        api: ApiObject {
            tag: "V2Neko_API".to_owned(),
            services: vec![
                "ReflectionService".to_owned(),
                "HandlerService".to_owned(),
                "LoggerService".to_owned(),
                "StatsService".to_owned(),
            ],
        },
        log: LogObject {
            log_level: "error".to_string(),
        },
        dns: DnsObject {
            server: config.dns.clone(),
        },
        inbounds: get_inbound_object(&config),
        outbound: bind.outbounds,
        policy: PolicyObject {
            system: SystemPolicyObject {
                stats_inbound_uplink: true,
                stats_inbound_downlink: true,
                stats_outbound_uplink: true,
                stats_outbound_downlink: true,
            },
        },
        routing: RoutingObject {
            domain_strategy: "mph".to_string(),
            domain_matcher: "AsIs".to_string(),
            rules: vec![RuleObject {
                rule_type: "field".to_string(),
                inbound_tag: vec!["V2Neko_API_INBOUND".to_string()],
                outbound_tag: "V2Neko_API".to_string(),
            }],
        },
        stats: StatsObject {},
    };
    serde_json::to_string_pretty(&config).unwrap()
}

fn parse_by_share_link_base64(link: &str) -> Result<String, ParseLinkError> {
    let config = read();
    if let Some(data) = link.to_lowercase().find("vmess://") {
        let decoded = general_purpose::STANDARD.decode(&link[data + 8..]);
        if decoded.is_err() {
            return Err(ParseLinkError {
                msg: decoded.err().unwrap().to_string(),
                code: super::error::ParseLinkErrorCode::Base64Error,
            });
        }
        let from_utf8 = String::from_utf8(decoded.unwrap()).unwrap();
        let json = serde_json::from_str::<Base64LinkObject>(&from_utf8);
        if json.is_err() {
            return Err(ParseLinkError {
                msg: json.err().unwrap().to_string(),
                code: super::error::ParseLinkErrorCode::JsonEror,
            });
        }
        let json = json.ok().unwrap();
        let outbound = Outbounds {
            outbounds: vec![OutboundObject {
                send_through: None,
                protocol: json.base64_type.clone(),
                settings: OutboundConfigurationObject::Vmess {
                    vnext: vec![VmessServerObject {
                        address: json.add,
                        port: json.port,
                        users: vec![UserObject {
                            id: json.id,
                            alter_id: Some(json.aid),
                            level: None,
                            security: json.scy.unwrap_or("auto".to_string()),
                        }],
                    }],
                },
                tag: "PROXY".to_string(),
                stream_settings: StreamSettingsObject {
                    network: json.net.clone(),
                    security: Some(json.tls),
                    tcp_settings: if json.net.as_str() == "tcp" {
                        match json.base64_type.as_str() {
                            "none" => Some(TcpObject {
                                header: TcpHeaderObject::NoneHeaderObject {
                                    tcp_type: "none".to_string(),
                                },
                            }),
                            "http" => Some(TcpObject {
                                header: TcpHeaderObject::HttpHeaderObject {
                                    tcp_type: "http".to_string(),
                                    request: None,
                                    response: None,
                                },
                            }),

                            _ => None,
                        }
                    } else {
                        None
                    },
                    kcp_settings: if json.net == "kcp" {
                        Some(KcpObject {
                            header: KcpHeaderObject {
                                kcp_type: json.base64_type,
                            },
                            mtu: None,
                            tti: None,
                            uplink_capacity: None,
                            downlink_capacity: None,
                            congestion: None,
                            read_buffer_size: None,
                            write_buffer_size: None,
                        })
                    } else {
                        None
                    },
                    ws_settings: if json.net == "ws" {
                        Some(WebSocketObject {
                            path: json.path.clone(),
                            headers: None,
                        })
                    } else {
                        None
                    },
                    http_settings: if json.net == "h2" {
                        Some(HttpObject {
                            host: vec![json.host.clone().unwrap()],
                            path: json.path.clone(),
                        })
                    } else {
                        None
                    },
                    ds_settings: None,
                    quic_settings: if json.net == "quic" {
                        Some(QUICObject {
                            security: json.host.unwrap(),
                            key: json.path.unwrap(),
                            header: None,
                        })
                    } else {
                        None
                    },
                    sockopt: Some(SockoptObject {
                        mark: 0,
                        tcp_fast_open: config.tcp_fast_open,
                        tproxy: "off".to_string(),
                    }),
                },
                proxy_settings: None,
                mux: MuxObject {
                    enabled: None,
                    concurrency: None,
                },
            }],
        };
        return Ok(generate(&outbound));
    }
    Err(ParseLinkError {
        msg: r#""#.to_string(),
        code: super::error::ParseLinkErrorCode::LinkError,
    })
}

fn generate_share_link_base64(outbound: &OutboundObject, name: &str) -> String {
    let OutboundConfigurationObject::Vmess { vnext } = &outbound.settings;
    let vnext = &vnext[0];
    let user_object = vnext.users[0].clone();
    let result = Base64LinkObject {
        v: 2.to_string(),
        ps: name.to_owned(),
        add: vnext.address.clone(),
        port: vnext.port,
        id: user_object.id,
        aid: user_object.alter_id.unwrap_or(0),
        scy: match user_object.security.as_str() {
            "auto" => None,
            _ => Some(user_object.security),
        },
        net: if outbound.stream_settings.network == "http" {
            "h2".to_owned()
        } else {
            outbound.stream_settings.network.clone()
        },
        base64_type: match outbound.stream_settings.network.as_str() {
            "tcp" => match outbound
                .stream_settings
                .tcp_settings
                .as_ref()
                .unwrap()
                .header
            {
                TcpHeaderObject::NoneHeaderObject { tcp_type: _ } => "none".to_string(),
                TcpHeaderObject::HttpHeaderObject {
                    tcp_type: _,
                    request: _,
                    response: _,
                } => "http".to_string(),
            },
            "kcp" => outbound
                .stream_settings
                .kcp_settings
                .as_ref()
                .unwrap()
                .header
                .kcp_type
                .clone(),
            "quic" => outbound
                .stream_settings
                .quic_settings
                .as_ref()
                .unwrap()
                .security
                .clone(),
            _ => "none".to_owned(),
        },
        host: match outbound.stream_settings.network.as_str() {
            "tcp" => {
                if let TcpHeaderObject::HttpHeaderObject {
                    tcp_type,
                    request,
                    response,
                } = &outbound
                    .stream_settings
                    .tcp_settings
                    .as_ref()
                    .unwrap()
                    .header
                {
                    Some(request.as_ref().unwrap().path[0].clone())
                } else {
                    None
                }
            }
            "ws" => {
                let mut map: HashMap<String, String> = HashMap::new();
                map.insert("Host".to_owned(), "".to_owned());
                Some(
                    outbound
                        .stream_settings
                        .ws_settings
                        .as_ref()
                        .unwrap()
                        .headers
                        .as_ref()
                        .unwrap_or(&map)["Host"]
                        .clone(),
                )
            }
            "http" => Some(
                outbound
                    .stream_settings
                    .http_settings
                    .as_ref()
                    .unwrap()
                    .host[0]
                    .clone(),
            ),
            "quic" => Some(
                outbound
                    .stream_settings
                    .quic_settings
                    .as_ref()
                    .unwrap()
                    .security
                    .clone(),
            ),
            _ => Some("".to_owned()),
        },
        path: match outbound.stream_settings.network.as_str() {
            "ws" => Some(
                outbound
                    .stream_settings
                    .ws_settings
                    .as_ref()
                    .unwrap()
                    .path
                    .as_ref()
                    .unwrap()
                    .clone(),
            ),
            "http" => Some(
                outbound
                    .stream_settings
                    .http_settings
                    .as_ref()
                    .unwrap()
                    .path
                    .as_ref()
                    .unwrap()
                    .clone(),
            ),
            "quic" => Some(
                outbound
                    .stream_settings
                    .quic_settings
                    .as_ref()
                    .unwrap()
                    .key
                    .clone(),
            ),
            _ => None,
        },
        tls: outbound.stream_settings.security.as_ref().unwrap().clone(),
        sni: None,
    };
    let raw_data = serde_json::to_string(&result).unwrap();
    let encoded_data = general_purpose::STANDARD.encode(raw_data);
    format!("vmess://{}", encoded_data)
}

#[cfg(test)]
mod tests {
    use crate::vmess::generate;

    use super::*;
    #[test]
    fn test_generate_v2config() {
        let a = generate(
            &serde_json::from_str(
                r#"{
    "outbounds": [
        {
            "mux": {
            },
            "protocol": "vmess",
            "sendThrough": "0.0.0.0",
            "settings": {
                "vnext": [
                    {
                        "address": "0kxedm1x8q8lksmj11.xingbayun.buzz",
                        "port": 12003,
                        "users": [
                            {
                                "id": "65a42bd8-cfe6-4cc5-ab47-04fdd4c1e799",
                                "security": "aes-128-gcm"
                            }
                        ]
                    }
                ]
            },
            "streamSettings": {
                "tlsSettings": {
                    "disableSystemRoot": false
                },
                "xtlsSettings": {
                    "disableSystemRoot": false
                }
            },
            "tag": "PROXY"
        }
    ]
}
"#,
            )
            .unwrap(),
        );
        println!("{}", a);
    }
    #[test]
    fn test_parse_link_base64() {
        let link = "vmess://ewogICJ2IjogIjIiLAogICJwcyI6ICIyIiwKICAiYWRkIjogIjIwLjI0LjczLjE2NCIsCiAgInBvcnQiOiA4MCwKICAiaWQiOiAiYzdjMWM5ODUtOTQyMS00ZDBmLWZhMTktMGVmZGE4MDM0M2FmIiwKICAiYWlkIjogMCwKICAibmV0IjogIndzIiwKICAidHlwZSI6ICJub25lIiwKICAiaG9zdCI6ICIiLAogICJwYXRoIjogIi8iLAogICJ0bHMiOiAibm9uZSIKfQ==";
        let a = parse_by_share_link_base64(link).unwrap();
        let json = serde_json::from_str::<ConfigJson>(a.as_str()).unwrap();
        // assert_eq!(a, "{\n  \"v\": \"2\",\n  \"ps\": \"2\",\n  \"add\": \"20.24.73.164\",\n  \"port\": 80,\n  \"id\": \"c7c1c985-9421-4d0f-fa19-0efda80343af\",\n  \"aid\": 0,\n  \"net\": \"ws\",\n  \"type\": \"none\",\n  \"host\": \"\",\n  \"path\": \"/\",\n  \"tls\": \"none\"\n}")
        println!("{}", a);
        let outbound = &json.outbound[0];
        let sss = generate_share_link_base64(outbound, "a");
        assert_eq!(link.to_owned(), sss)
    }
}
