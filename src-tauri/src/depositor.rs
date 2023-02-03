use crate::proxy::Proxy;
use directories::BaseDirs;
use rusqlite::{params, Connection};
use std::fs;

/// 通过id读取代理。
/// read proxy by id.
pub fn get_proxy_by_id(conn: &Connection, proxy_id: &str) -> Proxy {
    let mut stmt = conn
        .prepare(r#"SELECT * FROM proxies where proxy_id=?"#)
        .unwrap();
    let mut proxy_iter = stmt
        .query_map([proxy_id], |pair| {
            Ok(Proxy {
                proxy_id: pair.get(0)?,
                proxy_name: pair.get(1)?,
                proxy_type: pair.get(2)?,
                proxy_upload: pair.get(3).unwrap_or(0),
                proxy_download: pair.get(4).unwrap_or(0),
                proxy_delay: pair.get(5).unwrap_or(-1),
                proxy_config_path: pair.get(6)?,
                proxy_group:pair.get(7)?
            })
        })
        .unwrap();
    proxy_iter.next().unwrap().unwrap()
}

/// 向数据库中加入代理。
/// add a new proxy to the database.
pub fn push_proxy(conn: &Connection, proxy: &Proxy) {
    conn.execute("INSTER INTO proxies(proxy_id,proxy_name,proxy_type,proxy_upload,proxy_download,proxy_delay,proxy_config_path) 
    values (?,?,?,?,?,?,?)",params![proxy.proxy_id,proxy.proxy_name,proxy.proxy_type,proxy.proxy_upload,proxy.proxy_delay,proxy.proxy_config_path]).unwrap();
}

/// 获取存储在数据库中的的代理列表。
/// Get all proxies from the database.
pub fn get_proxy_list(connection: &Connection) -> Vec<Proxy> {
    let mut result = Vec::new();
    let mut stmt = connection.prepare(r#"SELECT * FROM proxies"#).unwrap();
    let proxy_iter = stmt
        .query_map([], |pair| {
            Ok(Proxy {
                proxy_id: pair.get(0)?,
                proxy_name: pair.get(1)?,
                proxy_type: pair.get(2)?,
                proxy_upload: pair.get(3).unwrap_or(0),
                proxy_download: pair.get(4).unwrap_or(0),
                proxy_delay: pair.get(5).unwrap_or(-1),
                proxy_config_path: pair.get(6)?,
                proxy_group:pair.get(7)?
            })
        })
        .unwrap();
    for i in proxy_iter {
        let j = i.unwrap();
        result.push(j)
    }
    result
}

/// 获取数据库的连接。将会在数据库不存在是建立数据库
/// get the connection from the database. Creates a new database when the database is not initialized.
pub fn init_database() -> Connection {
    let proj_dirs = BaseDirs::new().unwrap().config_dir().join("v2neko");
    fs::create_dir_all(&proj_dirs).unwrap();
    let mut conn = rusqlite::Connection::open(proj_dirs.join("proxyies.sqlite")).unwrap();
    init_proxys(&mut conn);
    conn
}

/// 初始化数据库
/// init the database.
fn init_proxys(conn: &mut Connection) {
    let mut stmt = conn
        .prepare(r#"SELECT count(*) FROM sqlite_master WHERE type="table" AND name = "proxies""#)
        .unwrap();
    let proxy_iter = stmt.query_map([], |pair| {
        if pair.get(0) == Ok(0) {
            conn.execute(
                "CREATE TABLE proxies(
                proxy_id varchar(36) PRIMARY KEY NOT NULL,
                proxy_name varchar(255) NOT NULL,
                proxy_type varchar(255) NOT NULL,
                proxy_upload int,
                proxy_download int,
                proxy_delay int,
                proxy_config varchar(65535) NOT NULL,
                proxy_group varchar(255),
            )",
                [],
            )
            .unwrap();
        }
        Ok(0)
    });
    for _ in proxy_iter {}
}
