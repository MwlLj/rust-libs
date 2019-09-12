use number_conv::array::u8arr;
use rust_parse::stream::tcp_block_with_errcode::{Code, CStreamBlockParse};
use super::{_BodyHead, Body};

use std::net::TcpStream;

#[derive(Debug)]
pub enum ResultCode {
    Success,
    CrcCheckFailed
}

struct CUnPackage {
}

impl CUnPackage {
    fn start<F>(&self, stream: TcpStream, f: &mut F, isCheckCrc: bool)
        where F: FnMut(&mut Body, ResultCode) -> bool {
        let mut parser = CStreamBlockParse::new(stream);
        let mut bodyHead = _BodyHead::default();
        let mut body = Body::default();
        parser.lines(8, &mut body, &mut |index: u64, data: Vec<u8>, b: &mut Body| -> (Code, u64) {
            // println!("{:?}", &data);
            if index == 0 {
                u8arr::u8arrTou64(data.as_slice(), &mut bodyHead.headCrc);
                return (Code::Continue, 8);
            } else if index == 1 {
                u8arr::u8arrTou64(data.as_slice(), &mut bodyHead.dataCrc);
                return (Code::Continue, 2);
            } else if index == 2 {
                u8arr::u8arrTou16(data.as_slice(), &mut bodyHead.protoNoLength);
                return (Code::Continue, 2);
            } else if index == 3 {
                u8arr::u8arrTou16(data.as_slice(), &mut bodyHead.messageIdLength);
                return (Code::Continue, 8);
            } else if index == 4 {
                u8arr::u8arrTou64(data.as_slice(), &mut bodyHead.paramLength);
                return (Code::Continue, 8);
            } else if index == 5 {
                u8arr::u8arrTou64(data.as_slice(), &mut bodyHead.extraLength);
                return (Code::Continue, 8);
            } else if index == 6 {
                u8arr::u8arrTou64(data.as_slice(), &mut bodyHead.dataLength);
                return (Code::Continue, bodyHead.protoNoLength as u64);
            } else if index == 7 {
                b.protoNo = data;
                return (Code::Continue, bodyHead.messageIdLength as u64);
            } else if index == 8 {
                b.messageId = data;
                return (Code::Continue, bodyHead.paramLength);
            } else if index == 9 {
                b.param = data;
                return (Code::Continue, bodyHead.extraLength);
            } else if index == 10 {
                b.extra = data;
                return (Code::Continue, bodyHead.dataLength);
            } else if index == 11 {
                b.data = data;
                if isCheckCrc {
                    // crc check
                    if self.crcCheck(&bodyHead, &b) {
                    } else {
                        return (Code::Error, 0);
                    }
                }
                return (Code::End, 0);
            }
            (Code::Failed, 0)
        }, &mut |b: &mut Body, code: Code| -> bool {
            let mut resultCode = ResultCode::Success;
            match code {
                Code::Error => {
                    resultCode = ResultCode::CrcCheckFailed;
                },
                Code::Failed => {
                    return false;
                },
                _ => {
                }
            }
            f(b, resultCode)
        });
    }
}

impl CUnPackage {
    fn crcCheck(&self, head: &_BodyHead, body: &Body) -> bool {
        let dataCrc = self.calcDataCrc(body);
        let headCrc = self.calcHeadCrc(dataCrc, head);
        if headCrc != head.headCrc
            && dataCrc != head.dataCrc {
            false
        } else {
            true
        }
    }

    fn calcHeadCrc(&self, dataCrc: u64, head: &_BodyHead) -> u64 {
        let mut v = Vec::new();
        u8arr::u64AppendTou8arr(dataCrc, 8, &mut v);
        u8arr::u64AppendTou8arr(head.protoNoLength as u64, 2, &mut v);
        u8arr::u64AppendTou8arr(head.messageIdLength as u64, 2, &mut v);
        u8arr::u64AppendTou8arr(head.paramLength as u64, 8, &mut v);
        u8arr::u64AppendTou8arr(head.extraLength as u64, 8, &mut v);
        u8arr::u64AppendTou8arr(head.dataLength as u64, 8, &mut v);
        crc::crc64::checksum_ecma(&v)
    }

    fn calcDataCrc(&self, body: &Body) -> u64 {
        let mut body = body.clone();
        let mut v = Vec::new();
        v.append(&mut body.protoNo);
        v.append(&mut body.messageId);
        v.append(&mut body.param);
        v.append(&mut body.extra);
        v.append(&mut body.data);
        crc::crc64::checksum_ecma(&v)
    }
}

impl CUnPackage {
    fn new() -> CUnPackage {
        CUnPackage{}
    }
}

pub fn listen<F>(stream: TcpStream, f: &mut F, isCheckCrc: bool)
    where F: FnMut(&mut Body, ResultCode) -> bool {
    CUnPackage::new().start(stream, f, isCheckCrc);
}
