#[derive(Default, Debug, Clone)]
pub struct Body {
    pub protoNo: Vec<u8>,
    pub messageId: Vec<u8>,
    pub param: Vec<u8>,
    pub extra: Vec<u8>,
    pub data: Vec<u8>
}

#[derive(Default)]
struct _Body {
    headCrc: u64,
    dataCrc: u64,
    protoNoLength: u16,
    messageIdLength: u16,
    paramLength: u64,
    extraLength: u64,
    dataLength: u64,
    protoNo: Vec<u8>,
    messageId: Vec<u8>,
    param: Vec<u8>,
    extra: Vec<u8>,
    data: Vec<u8>
}

#[derive(Default)]
struct _BodyHead {
    headCrc: u64,
    dataCrc: u64,
    protoNoLength: u16,
    messageIdLength: u16,
    paramLength: u64,
    extraLength: u64,
    dataLength: u64
}

#[derive(Default)]
struct _BodyLength {
    protoNoLength: usize,
    messageIdLength: usize,
    paramLength: usize,
    extraLength: usize,
    dataLength: usize
}

pub mod pack;
pub mod unpack;
