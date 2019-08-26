use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
	// 216.52.148.134:27961 urtctf
	//let mut _socket1 = UdpSocket::bind("216.52.148.134:27961").expect("couldnt bind pkw");
	//let mut _socket2 = UdpSocket::bind("216.52.148.134:27961")?;
	let mut _socket = UdpSocket::bind("127.0.0.1:34254")?;

	Ok(())
}
