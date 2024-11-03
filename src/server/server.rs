use tokio::net::{TcpListener, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::UnboundedSender;
use std::io::{self, Write};

pub async fn server_start() {
    // 토키오 리시버, 샌더 생성
    let (command_sender, command_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (udp_command_sender, udp_command_receiver) = tokio::sync::mpsc::unbounded_channel();
    io::stdout().flush().unwrap();


    tokio::join!(
        run_server(command_sender),
        run_udp_server(udp_command_sender)
    );
}

async fn run_udp_server(command_sender: UnboundedSender<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("run_udp_server 함수가 호출되었습니다.");
    io::stdout().flush().unwrap();

    // UDP 소켓을 7878 포트에 바인딩합니다.
    let socket = UdpSocket::bind("127.0.0.1:7878").await?;
    println!("UDP 서버가 7878 포트에서 대기 중입니다...");
    io::stdout().flush().unwrap();
    let mut buffer = [0; 1024];

    loop {
        // 데이터 수신 (클라이언트 주소도 함께 수신)
        let (size, client_addr) = match socket.recv_from(&mut buffer).await {
            Ok((size, addr)) => (size, addr),
            Err(e) => {
                eprintln!("데이터 수신 중 오류 발생: {}", e);
                continue;
            }
        };

        let received_data = String::from_utf8_lossy(&buffer[..size]);
        println!("{}로부터 데이터 수신: {}", client_addr, received_data);

        // Bevy로 메시지 전달
        if let Err(e) = command_sender.send(received_data.to_string()) {
            eprintln!("메시지 전송 중 오류 발생: {}", e);
        }

        // 클라이언트로 에코 응답
        if let Err(e) = socket.send_to(&buffer[..size], &client_addr).await {
            eprintln!("응답 전송 중 오류 발생: {}", e);
        }
    }
}

async fn run_server(command_sender: UnboundedSender<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("run_server 함수가 호출되었습니다.");
    io::stdout().flush().unwrap();

    let listener = match TcpListener::bind("127.0.0.1:7879").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("서버 바인딩 중 오류 발생: {}", e);
            return Err(Box::new(e));
        }
    };
    println!("서버가 7878 포트에서 대기 중입니다...");
    io::stdout().flush().unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;
        let sender = command_sender.clone();

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            match socket.read(&mut buffer).await {
                Ok(n) if n == 0 => return, // 연결 종료
                Ok(n) => {
                    let received_data = String::from_utf8_lossy(&buffer[..n]);
                    println!("클라이언트로부터 데이터 수신: {}", received_data);

                    // Bevy로 메시지 전달
                    if let Err(e) = sender.send(received_data.to_string()) {
                        eprintln!("메시지 전송 중 오류 발생: {}", e);
                    }

                    // 에코 응답
                    if let Err(e) = socket.write_all(&buffer[..n]).await {
                        eprintln!("응답 전송 중 오류 발생: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("데이터 수신 중 오류 발생: {}", e);
                }
            }
        });
    }
}