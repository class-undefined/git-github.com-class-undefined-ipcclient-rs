use msgpack_simple::{MapElement, MsgPack};

use crate::types::IpcMessage;

pub struct IpcHeader {
    pub compress: bool,
}

impl IpcHeader {
    pub fn new() -> IpcHeader {
        IpcHeader { compress: false }
    }
}

impl Default for IpcHeader {
    fn default() -> Self {
        IpcHeader { compress: false }
    }
}
impl IpcMessage for IpcHeader {
    fn encode(self) -> MsgPack {
        let map = MsgPack::Map(vec![MapElement {
            key: MsgPack::String(String::from("compress")),
            value: MsgPack::Boolean(self.compress),
        }]);
        map
    }

    fn decode(pack: MsgPack) -> Self {
        let mut header = IpcHeader::new();
        if let MsgPack::Map(elements) = pack {
            for element in elements {
                if let MsgPack::String(key) = element.key {
                    if key == "compress" {
                        if let MsgPack::Boolean(value) = element.value {
                            header.compress = value;
                        }
                    }
                }
            }
        }
        header
    }
}
