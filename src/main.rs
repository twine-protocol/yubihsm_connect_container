use anyhow::Result;

fn init_hsm() -> Result<()> {
  let connector = yubihsm::Connector::usb(&yubihsm::UsbConfig {
    serial: None,
    timeout_ms: 60000
  });

  let creds = yubihsm::Credentials::default();
  let client = yubihsm::Client::open(connector, creds, true)?;

  // generate a new connection key
  dbg!(client.device_info()?);

  Ok(())
}

fn run_server() -> Result<()> {
  let port = 12345;
  let connector = yubihsm::Connector::usb(&yubihsm::UsbConfig {
    serial: None,
    // generating RSA keys takes a while and can timeout
    // https://github.com/iqlusioninc/yubihsm.rs/issues/267
    timeout_ms: 60000
  });

  let server = yubihsm::connector::http::Server::new(
    &yubihsm::HttpConfig {
      addr: "0.0.0.0".into(),
      port,
      timeout_ms: 60000
    },
    connector
  )?;

  println!("Listening on port {}", port);
  server.run()?;
  Ok(())
}

fn main() -> Result<()> {
  // if we have an "init" argument, we'll initialize the HSM
  if std::env::args().nth(1) == Some("init".into()) {
    init_hsm()
  } else {
    run_server()
  }
}
