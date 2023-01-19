use std::collections::HashMap;

use crate::config::read;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct LogObject {
    #[serde(alias = "loglevel")]
    log_level: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ApiObject {
    tag: String,
    services: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct DnsObject {
    server: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct InboundObject {
    port: i32,
    listen: String,
    protocol: String,
    settings: InboundConfigurationObject,
    tag: String,
    sniffing: SniffingObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
enum InboundConfigurationObject {
    Socks {
        auth: String,
        udp: bool,
        ip: String,
        user_level: i32,
    },
    Http {
        timeout: i32,
        accounts: Option<HttpUserObject>,
        allow_transparent: bool,
        level: i32,
    },
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct HttpUserObject {
    user: String,
    pass: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct OutboundObject {
    #[serde(rename = "sendThrough")]
    send_through: String,
    protocol: String,
    settings: OutboundConfigurationObject,
    tag: String,

    stream_settings: StreamSettingsObject,
    proxy_settings: Option<ProxySettingsObject>,
    mux: MuxObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct MuxObject {
    enabled: Option<bool>,
    concurrency: Option<i32>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ProxySettingsObject {
    tag: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(untagged)]
enum OutboundConfigurationObject {
    #[serde(rename = "vnext")]
    Vmess { vnext: VmessServerObject },
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StreamSettingsObject {
    network: Option<String>,
    security: Option<String>,
    tcp_settings: Option<TcpObject>,
    kcp_settings: Option<KcpObject>,
    ws_settings: Option<WebSocketObject>,
    http_settings: Option<HttpObject>,
    ds_settings: Option<DomainSocketObject>,
    quic_settings: Option<QUICObject>,
    sockopt: Option<SockoptObject>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct QUICObject {
    security: String,
    key: String,
    header: QUICHeaderObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct QUICHeaderObject {
    #[serde(rename = "type")]
    quic_type: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct DomainSocketObject {
    path: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct HttpObject {
    host: Vec<String>,
    path: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct WebSocketObject {
    path: String,
    headers: HashMap<String, String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct KcpObject {
    mtu: i32,
    tti: i32,
    uplink_capacity: i32,
    downlink_capacity: i32,
    congestion: bool,
    read_buffer_size: i32,
    write_buffer_size: i32,
    header: KcpHeaderObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct KcpHeaderObject {
    #[serde(rename = "type")]
    kcp_type: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct TcpObject {
    #[serde(rename = "type")]
    tcp_type: String,
    request: Option<HTTPRequestObject>,
    response: Option<HTTPResponseObject>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct HTTPRequestObject {
    version: String,
    method: String,
    path: Vec<String>,
    headers: HashMap<String, Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct HTTPResponseObject {
    version: String,
    status: String,
    reason: String,
    headers: HashMap<String, Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SockoptObject {
    mark: i32,
    tcp_fast_open: bool,
    tproxy: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct VmessServerObject {
    address: String,
    port: i32,
    users: Vec<UserObject>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UserObject {
    id: String,
    alter_id: Option<i32>,
    level: Option<i32>,
    security: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SniffingObject {
    #[serde(alias = "destOverride")]
    dest_override: Vec<String>,
    enabled: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct PolicyObject {
    system: SystemPolicyObject,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SystemPolicyObject {
    #[serde(alias = "statsInboundUplink")]
    stats_inbound_uplink: bool,
    #[serde(alias = "statsInboundDownlink")]
    stats_inbound_downlink: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RoutingObject {
    #[serde(alias = "domainStrategy")]
    domain_strategy: String,
    #[serde(alias = "domainMatcher")]
    domain_matcher: String,
    rules: Vec<RuleObject>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RuleObject {
    #[serde(rename = "type")]
    rule_type: String,
    #[serde(alias = "inboundTag")]
    inbound_tag: Vec<String>,
    #[serde(alias = "outboundTag")]
    outbound_tag: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Outbounds {
    outbounds: Vec<OutboundObject>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct StatsObject {}

pub fn generate(outbound: &str) -> String {
    let config = read();
    let binding = serde_json::from_str::<Outbounds>(outbound).unwrap();
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
        dns: DnsObject { server: config.dns },
        inbounds: vec![InboundObject {
            port: config.socks_port,
            listen: config.socks_bind,
            protocol: "socks".to_string(),
            settings: InboundConfigurationObject::Socks {
                auth: "noauth".to_string(),
                udp: true,
                ip: "127.0.0.1".to_string(),
                user_level: 0,
            },
            tag: "socks_IN".to_string(),
            sniffing: SniffingObject {
                dest_override: vec!["http".to_string(), "tls".to_string(), "fakedns".to_string()],
                enabled: true,
            },
        }],
        outbound: binding.outbounds,
        policy: PolicyObject {
            system: SystemPolicyObject {
                stats_inbound_uplink: true,
                stats_inbound_downlink: true,
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

fn generate_share_link(link:&str){
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_v2config() {
        let a = generate(
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
        );
        println!("{}", a);
    }
}
