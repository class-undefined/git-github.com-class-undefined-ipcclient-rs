use std::{io::Write, os::unix::net::UnixStream};

use crate::{header::IpcHeader, request::IpcRequest, types::IpcMessage};

pub struct IpcClient {
    pub id: String,
    pub socket: UnixStream,
}

impl IpcClient {
    pub fn new(socket: UnixStream) -> IpcClient {
        let uuid = uuid::Uuid::new_v4().to_string();
        IpcClient { socket, id: uuid }
    }

    /// Create a new IpcClient from a socket path
    pub fn from_socket_path(path: &str) -> IpcClient {
        let socket = UnixStream::connect(path).unwrap();
        IpcClient::new(socket)
    }

    /// Create a new IpcClient from the environment variable `name`
    pub fn from_sys_env(name: &str) -> Result<IpcClient, String> {
        let path = std::env::var(name);
        if path.is_err() {
            return Err(format!("Environment variable {} not found", name));
        }
        let path = path.unwrap();
        if std::path::Path::exists(std::path::Path::new(&path)) {
            return Err(format!("Path {} does not exist", path));
        }
        Ok(IpcClient::from_socket_path(&path))
    }

    pub fn send<T: IpcMessage>(&mut self, path: &str, data: T) -> std::io::Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let header = IpcHeader::default();
        let body = Some(data);
        let request = IpcRequest::new(id, self.id.clone(), path.to_string(), header, body);
        let buffer_body = request.encode().encode();
        let size = buffer_body.len();
        let mut buffer = Vec::with_capacity(8 + buffer_body.len());
        buffer.extend_from_slice(&size.to_be_bytes());
        buffer.extend_from_slice(&buffer_body);
        self.socket.write_all(&buffer).unwrap();
        self.socket.flush()?;
        Ok(())
    }

    /// Send a string to the server
    /// # Example
    /// ```rs
    /// let mut client = IpcClient::from_socket_path("/tmp/ipcserver.sock");
    /// client.send_string("/demo/hello", "12345".to_string());
    /// ```
    pub fn send_string(&mut self, path: &str, data: String) -> std::io::Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let header = IpcHeader::default();
        let body = Some(data);
        let request = IpcRequest::new(id, self.id.clone(), path.to_string(), header, body);
        let buffer_body = request.encode().encode();
        let size = buffer_body.len();
        let mut buffer = Vec::with_capacity(8 + buffer_body.len());
        buffer.extend_from_slice(&size.to_be_bytes());
        buffer.extend_from_slice(&buffer_body);
        self.socket.write_all(&buffer).unwrap();
        self.socket.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ipc_client() {
        let mut client = super::IpcClient::from_socket_path("/tmp/ipcserver.sock");
        let _ = client.send_string("/demo/hello", "12345".to_string());
    }
}
