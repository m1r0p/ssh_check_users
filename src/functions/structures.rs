/////// structures
#[derive(Debug)]

pub struct User {
    pub username: String,
    pub password: bool,
    pub uid: i32,
    pub gid: i32,
    pub gecos: String,
    pub home: String,
    pub shell: String,
}
