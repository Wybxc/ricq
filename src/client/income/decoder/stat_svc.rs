use crate::jce::*;
use bytes::{Bytes, Buf};
use jce_struct::Jce;
use crate::client::errors::RQError;

pub fn decode_client_register_response(payload: &[u8]) -> SvcRespRegister {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: RequestDataVersion2 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut a = data.map.remove("SvcRespRegister").unwrap();
    let mut b = a.remove("QQService.SvcRespRegister").unwrap();
    b.advance(1);
    Jce::read_from_bytes(&mut b)
}

pub fn decode_dev_list_response(payload: &[u8]) -> Result<Vec<SvcDevLoginInfo>,RQError> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: RequestDataVersion2 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut req = data.map.remove("SvcRspGetDevLoginInfo").unwrap();
    let mut msg = req.remove("QQService.SvcRspGetDevLoginInfo").unwrap();
    let mut rsp = Jce::new(&mut msg);
    let d: Vec<SvcDevLoginInfo> = rsp.get_by_tag(4);
    if d.len() > 0 {
        return Ok(d);
    }
    let d: Vec<SvcDevLoginInfo> = rsp.get_by_tag(5);
    if d.len() > 0 {
        return Ok(d);
    }
    let d: Vec<SvcDevLoginInfo> = rsp.get_by_tag(6);
    if d.len() > 0 {
        return Ok(d);
    }
    return Err(RQError::Decode("decode_dev_list_response".to_string()));
}