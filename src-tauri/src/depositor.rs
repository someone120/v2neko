use rusqlite::{params, Connection};
use std::fs;
use crate::proxy::Proxy;


/// 将字节数转换为更适合人类阅读的形式
/// make the bytes more readable.
/// # Examples
///
/// ```rust
/// let a = 1024;
/// assert_eq!(byte_to_human_readable(&a),"1KB");
/// ```
fn _byte_to_human_readable(byte_count: &i64) -> String {
    let unit = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut resule = String::new();
    let mut j = byte_count.clone() as f64;
    for i in unit.iter() {
        j /= 1024.0;
        if j < 1.0 {
            resule = format!("{:.2} {}", j * 1024.0, i);
            break;
        }
    }
    resule
}

/// 通过id读取代理。
/// read proxy by id.
pub fn get_proxy_by_id(conn: &Connection, proxy_id: i32) -> Proxy {
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
            })
        })
        .unwrap();
    proxy_iter.next().unwrap().unwrap()
}

/// 向数据库中加入代理。
/// add a new proxy to the database.
pub fn _push_proxy(conn: &Connection, proxy: &Proxy) {
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
    fs::create_dir_all("~/.config/v2neko").unwrap();
    let mut conn = rusqlite::Connection::open("~/.config/v2neko/proxyies.sqlite").unwrap();
    init(&mut conn);
    conn
}

/// 初始化数据库
/// init the database.
fn init(conn: &mut Connection) {
    let mut stmt = conn
        .prepare(r#"SELECT count(*) FROM sqlite_master WHERE type="table" AND name = "proxies""#)
        .unwrap();
    let proxy_iter = stmt.query_map([], |pair| {
        if pair.get(0) == Ok(0) {
            conn.execute(
                "CREATE TABLE proxies(
                proxy_id int PRIMARY KEY NOT NULL,
                proxy_name varchar(255) NOT NULL,
                proxy_type varchar(255) NOT NULL,
                proxy_upload int,
                proxy_download int,
                proxy_delay int,
                proxy_config varchar(65535) NOT NULL
            )",
                [],
            )
            .unwrap();
        }
        Ok(0)
    });
    for _ in proxy_iter {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn byte_to_human_readable_test_kb() {
        let a = 2049;
        assert_eq!(_byte_to_human_readable(&a), "2.00 KB");
    }
    #[test]
    fn byte_to_human_readable_test_mb() {
        let a = 2048 * 1024;
        assert_eq!(_byte_to_human_readable(&a), "2.00 MB");
    }

    #[test]
    fn byte_to_human_readable_test_gb() {
        let a = 2048 * 1024 * 1024;
        assert_eq!(_byte_to_human_readable(&a), "2.00 GB");
    }
    #[test]
    fn byte_to_human_readable_test_tb() {
        let a = 2048 * 1024 * 1024 * 1024;
        assert_eq!(_byte_to_human_readable(&a), "2.00 TB");
    }
}
