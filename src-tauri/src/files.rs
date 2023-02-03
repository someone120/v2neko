use std::{
    fs,
    io::{self, ErrorKind},
};

use directories::BaseDirs;

pub fn write(path: &str, data: &str) -> io::Result<()> {
    Ok(if let Some(proj_dirs) = BaseDirs::new() {
        let mut pa = proj_dirs.config_dir().join("v2neko");
        fs::create_dir_all(&pa)?;
        pa = pa.join(path);
        fs::write(pa, data)?
    })
}

// pub fn read(path: &str) -> io::Result<String> {
//     if let Some(proj_dirs) = BaseDirs::new() {
//         let mut pa = proj_dirs.config_dir().join("v2neko");
//         fs::create_dir_all(&pa)?;
//         pa = pa.join(path);
//         Ok(fs::read_to_string(pa)?)
//     } else {
//         Err(io::Error::new(
//             ErrorKind::NotFound,
//             "not found config directory",
//         ))
//     }
// }
pub fn read(path: &str) -> io::Result<String> {
    let proj_dirs = BaseDirs::new()
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "not found config directory"))?;
    let mut pa = proj_dirs.config_dir().join("v2neko");
    fs::create_dir_all(&pa)?;
    pa = pa.join(path);
    Ok(fs::read_to_string(pa)?)
}
