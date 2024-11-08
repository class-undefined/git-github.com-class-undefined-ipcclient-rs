use msgpack_simple::MsgPack;

pub trait IpcMessage {
    fn encode(self) -> MsgPack;
    fn decode(msgpack: MsgPack) -> Self;
}

impl IpcMessage for String {
    fn encode(self) -> MsgPack {
        MsgPack::String(self)
    }

    fn decode(msgpack: MsgPack) -> Self {
        if let MsgPack::String(value) = msgpack {
            value
        } else {
            panic!("Invalid string")
        }
    }
}
