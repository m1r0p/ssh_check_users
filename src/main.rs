mod functions;


use functions::*;
use std::env;
extern crate ipnet;
use ipnet::Ipv4Net;
use std::collections::HashMap;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }
    if args.len() > 2 {
        panic!("too much arguments");
    }

    let net4 = String::from(&args[1]);
    let net4: Ipv4Net = net4.trim().parse().expect("Please type a subnet X.X.X.X/X");
    let iplist = get_iplist(net4);
    let mut users_dataset: HashMap<String, Vec<String>> = HashMap::new();
    for ip in iplist {
        println!("trying IP - {}", &ip);
        let conn: Result<String, Box<dyn Error>> = match get_raw_data(&ip) {
            Ok(tpl) => Ok(tpl),
            Err(_) => continue,
        };

        //let parsed_users: Vec<User> = parse_users(&conn.unwrap());
        let end_data: Result<Vec<User>, Box<dyn Error>> = parse_users(&conn.unwrap());
        let mut wrong_users: Vec<String> = Vec::new();
        match end_data {
            Ok(parsed_users) => {
                for p_user in &parsed_users {
                    if SHELLS.contains(&p_user.shell.as_str())
                        && !ALLOWED_USERS.contains(&p_user.username.as_str())
                    {
                        wrong_users.push(p_user.username.to_string());
                    }
                }
                if wrong_users.len() > 0 {
                    users_dataset.insert(ip.to_string(), wrong_users);
                }
            }
            Err(err) => println!("{:?}", err),
        }
    }
    println!("######################RESULTS#########################\n");
    if users_dataset.len() > 0 {
        for (ip, users) in &users_dataset {
            println!("host - {:?}, users - {:?}", &ip, &users);
        }
    } else {
        println!("There are no illegal users in {} subnet", &args[1]);
    }
}
