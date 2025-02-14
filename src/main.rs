use anyhow::Result;

fn connector() -> Result<yubihsm::Connector> {
  let connector = yubihsm::Connector::usb(&yubihsm::UsbConfig {
    serial: None,
    timeout_ms: 60000
  });

  Ok(connector)
}

fn run_server() -> Result<()> {
  let port = 12345;
  let connector = connector()?;

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
  run_server()
}
