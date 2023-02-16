pub mod config;
pub mod structures;
pub use config::{ALLOWED_USERS, CREDS, SHELLS};
pub use structures::User;

extern crate ipnet;
use ipnet::Ipv4Net;

use ssh2::Session;
use std::error::Error;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>());
//}

pub fn get_iplist(net4: Ipv4Net) -> Vec<String> {
    let mut iplist: Vec<String> = Vec::new();
    let subnets32 = net4
        .subnets(32)
        .expect("PrefixLenError: new prefix length cannot be shorter than existing");
    for subnet32 in subnets32 {
        iplist.push(subnet32.to_string().trim_end_matches("/32").to_string());
    }
    if iplist.len() > 2 {
        iplist.remove(0);
        iplist.remove(iplist.len() - 1);
    }

    return iplist;
}

pub fn get_raw_data(ip: &String) -> Result<String, Box<dyn Error>> {
    let mut ip: String = ip.clone();
    ip.push_str(":22");
    let ip_soc: SocketAddr = ip.parse().expect("Unable to parse socket address");
    let conn_wait = Duration::new(1, 0);
    let mut raw_data = String::new();

    for cred in &CREDS {
        let tcp = match TcpStream::connect_timeout(&ip_soc, conn_wait) {
            Ok(conn_success) => conn_success,
            Err(err) => return Err(Box::new(err)),
        };

        let mut sess = Session::new().unwrap();
        sess.set_timeout(6000);
        sess.set_tcp_stream(tcp);
        match sess.handshake() {
            Ok(kk) => kk,
            Err(err) => return Err(Box::new(err)),
        };

        match sess.userauth_password(cred[0], cred[1]) {
            Ok(auth_success) => auth_success,
            Err(_) => {
                println!("wrong credential {:?}", cred);
                continue;
            }
        };
        println!("SUCCESS AUTH WITH {:?} <===========================", cred);
        let mut channel = match sess.channel_session() {
            Ok(channel_success) => channel_success,
            Err(err) => return Err(Box::new(err)),
        };
        //channel.exec("cat /etc/passwd").unwrap();
        match channel.exec("cat /etc/passwd") {
            Ok(kk) => kk,
            Err(err) => return Err(Box::new(err)),
        };
        match channel.read_to_string(&mut raw_data) {
            Ok(kk) => kk,
            Err(err) => return Err(Box::new(err)),
        };
        if raw_data.len() == 0 {
            return Err("raw data is empty".into());
        }
        break;
    }

    if raw_data.len() > 0 {
        return Ok(raw_data);
    } else {
        return Err("something wrong".into());
    }
}

pub fn parse_users(raw_data: &String) -> Result<Vec<User>, Box<dyn Error>> {
    let data_string: String = raw_data.clone();
    let mut parsed_users: Vec<User> = Vec::new();
    let data: Vec<&str> = data_string.split("\n").collect();
    for row in &data {
        let user_attrs: Vec<&str> = row.split(":").collect();
        if user_attrs.len() == 7 {
            let user_struct: User = User {
                username: user_attrs[0].to_string(),
                password: true,
                uid: FromStr::from_str(user_attrs[2]).unwrap(),
                gid: FromStr::from_str(user_attrs[3]).unwrap(),
                gecos: user_attrs[4].to_string(),
                home: user_attrs[5].to_string(),
                shell: user_attrs[6].to_string(),
            };
            parsed_users.push(user_struct);
        }
    }
    if parsed_users.len() > 1 {
        return Ok(parsed_users);
    } else {
        return Err("output data doesn't contain users".into());
    }
}
