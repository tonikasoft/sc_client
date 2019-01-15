#[derive(Debug)]
pub enum ScClientError {
    OSC(String),
    Server(String),
}
