

use std::io::{Error, ErrorKind};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

use crate::backend::{scal::Scal, user::User};

struct session {
 scal:Scal,
 users:Vec<User>,
}
trait SessionUser {

}
impl session {
  pub async fn start() -> std::io::Result<()>  {
    let listener = TcpListener::bind("127.0.0.1:18191").await?;

    loop {
      let (mut socket, _) = listener.accept().await?;
      welcome_client(socket).await?;
    }
  }
}
async fn welcome_client(mut socket:TcpStream) -> std::io::Result<()> {
  let mut buffer = String::new();
  socket.write(format!("HI {}", env!("CARGO_PKG_VERSION")).as_bytes()).await?;
  socket.read_to_string(&mut buffer).await?;
  if !(buffer.starts_with("HI ") && &buffer[3..] == env!("CARGO_PKG_VERSION")) {
    return Err(Error::new(ErrorKind::Other, "두 rascal의 버전이 일치하지 않습니다. 혹은 잘못된 페킷임니다"));
  }
  Ok(())
}
