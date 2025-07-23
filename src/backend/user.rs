use tokio::net::TcpStream;

use crate::backend::group::Group;

pub struct User {
    name: String,
    id: usize,
    password: String,
    groups: Vec<Group>,
    rsa_pubkey:[char; 256]
}
pub struct SessionUser {
  user:User,
  socket:TcpStream
}
