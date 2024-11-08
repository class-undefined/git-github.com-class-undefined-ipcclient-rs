use msgpack_simple::{MapElement, MsgPack};

use crate::{header::IpcHeader, types::IpcMessage};

pub struct IpcRequest<T: IpcMessage> {
    pub id: String,
    pub client_id: String,
    pub path: String,
    pub header: IpcHeader,
    pub body: Option<T>,
}

impl<T: IpcMessage> IpcRequest<T> {
    pub fn new(
        id: String,
        client_id: String,
        path: String,
        header: IpcHeader,
        body: Option<T>,
    ) -> IpcRequest<T> {
        IpcRequest {
            id,
            client_id,
            path,
            header,
            body,
        }
    }
}

impl<T: IpcMessage> IpcMessage for IpcRequest<T> {
    fn encode(self) -> MsgPack {
        let mut body = MsgPack::Nil;
        if self.body.is_some() {
            body = self.body.unwrap().encode();
        }
        let map = MsgPack::Map(vec![
            MapElement {
                key: MsgPack::String(String::from("id")),
                value: MsgPack::String(self.id),
            },
            MapElement {
                key: MsgPack::String(String::from("clientId")),
                value: MsgPack::String(self.client_id),
            },
            MapElement {
                key: MsgPack::String(String::from("path")),
                value: MsgPack::String(self.path),
            },
            MapElement {
                key: MsgPack::String(String::from("header")),
                value: self.header.encode(),
            },
            MapElement {
                key: MsgPack::String(String::from("body")),
                value: body,
            },
        ]);
        map
    }

    fn decode(msgpack: MsgPack) -> Self {
        let mut request = IpcRequest::new(
            String::from(""),
            String::from(""),
            String::from(""),
            IpcHeader::default(),
            None,
        );
        if let MsgPack::Map(elements) = msgpack {
            for element in elements {
                if let MsgPack::String(key) = element.key {
                    match key.as_str() {
                        "id" => request.id = element.value.as_string().unwrap(),
                        "clientId" => request.client_id = element.value.as_string().unwrap(),
                        "path" => request.path = element.value.as_string().unwrap(),
                        "header" => request.header = IpcHeader::decode(element.value),
                        "body" => request.body = Some(T::decode(element.value)),
                        _ => panic!("Unknown key: {}", key),
                    }
                }
            }
        }

        request
    }
}
