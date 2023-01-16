use std::{
    io::{BufRead, BufReader},
    process::ChildStdout,
};

pub fn new_ws(port: i64, buf: &mut BufReader<ChildStdout>)->() {
    ws::listen(format!("127.0.0.1:{}",port), |out| {
        let mut line = String::new();
        while let Ok(_) = buf.read_line(&mut line) {
            out.send(line.as_str()).unwrap();
        }
        move |msg| out.send(msg)
    }).unwrap()
}
