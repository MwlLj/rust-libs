use super::{Body, _BodyLength};

use number_conv::array::u8arr;

struct CPack {
}

impl CPack {
    fn pack(&self, body: &mut Body) -> Vec<u8> {
        let mut length: _BodyLength = _BodyLength{
            protoNoLength: body.protoNo.len(),
            messageIdLength: body.messageId.len(),
            paramLength: body.param.len(),
            extraLength: body.extra.len(),
            dataLength: body.data.len()
        };
        let mut buf: Vec<u8> = Vec::new();
        let (dataCrc, mut dataBuf) = self.calcDataCrc(body);
        let (headCrc, mut headBuf) = self.calcHeadCrc(dataCrc, length);
        // join headCrc to 4byte
        u8arr::u64AppendTou8arr(headCrc, 8, &mut buf);
        // append head vec
        buf.append(&mut headBuf);
        // append data vec
        buf.append(&mut dataBuf);
        buf
    }
}

impl CPack {
    fn calcHeadCrc(&self, dataCrc: u64, length: _BodyLength) -> (u64, Vec<u8>) {
        let mut v = Vec::new();
        u8arr::u64AppendTou8arr(dataCrc, 8, &mut v);
        u8arr::u64AppendTou8arr(length.protoNoLength as u64, 2, &mut v);
        u8arr::u64AppendTou8arr(length.messageIdLength as u64, 2, &mut v);
        u8arr::u64AppendTou8arr(length.paramLength as u64, 8, &mut v);
        u8arr::u64AppendTou8arr(length.extraLength as u64, 8, &mut v);
        u8arr::u64AppendTou8arr(length.dataLength as u64, 8, &mut v);
        (crc::crc64::checksum_ecma(&v), v)
    }

    fn calcDataCrc(&self, body: &mut Body) -> (u64, Vec<u8>) {
        let mut v = Vec::new();
        v.append(&mut body.protoNo);
        v.append(&mut body.messageId);
        v.append(&mut body.param);
        v.append(&mut body.extra);
        v.append(&mut body.data);
        (crc::crc64::checksum_ecma(&v), v)
    }
}

impl CPack {
    fn new() -> CPack {
        CPack{}
    }
}

pub fn pack(body: &mut Body) -> Vec<u8> {
    CPack::new().pack(body)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn packTest() {
        let pack = CPack::new();
        let buf = pack.pack(&mut Body{
            protoNo: Vec::from("v1.0".to_string()),
            messageId: Vec::from("123456".to_string()),
            param: Vec::from("".to_string()),
            extra: Vec::from("".to_string()),
            data: Vec::from("hello".to_string())
        });
        println!("{:?}", &buf);
    }
}
