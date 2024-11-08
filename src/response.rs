use msgpack_simple::{MapElement, MsgPack};

use crate::{state::IpcStatus, types::IpcMessage};

pub struct IpcResponse<T: IpcMessage> {
    pub code: IpcStatus,
    pub message: String,
    pub data: Option<T>,
}

impl<T: IpcMessage> IpcResponse<T> {
    pub fn new(code: IpcStatus, message: String, data: Option<T>) -> IpcResponse<T> {
        IpcResponse {
            code,
            message,
            data,
        }
    }

    pub fn is_normal(&self) -> bool {
        match self.code {
            IpcStatus::OK => true,
            _ => false,
        }
    }
}

impl<T: IpcMessage> IpcMessage for IpcResponse<T> {
    fn encode(self) -> MsgPack {
        let mut data = MsgPack::Nil;
        if let Some(d) = self.data {
            data = d.encode();
        }
        let map = MsgPack::Map(vec![
            MapElement {
                key: MsgPack::String(String::from("code")),
                value: MsgPack::Int(self.code as i64),
            },
            MapElement {
                key: MsgPack::String(String::from("message")),
                value: MsgPack::String(self.message),
            },
            MapElement {
                key: MsgPack::String(String::from("data")),
                value: data,
            },
        ]);
        map
    }

    fn decode(msgpack: MsgPack) -> Self {
        let mut request = IpcResponse::new(IpcStatus::BadRequest, String::from(""), None);
        if let MsgPack::Map(elements) = msgpack {
            for element in elements {
                if let MsgPack::String(key) = element.key {
                    match key.as_str() {
                        "code" => request.code = IpcStatus::from(element.value.as_int().unwrap()),
                        "message" => request.message = element.value.as_string().unwrap(),
                        "data" => request.data = Some(T::decode(element.value)),
                        _ => panic!("Unknown key: {}", key),
                    }
                }
            }
        }

        request
    }
}
