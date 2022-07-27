use std::collections::HashMap;
use anyhow::{Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<'a, T> {
    pub code: usize,
    pub data: T,
    pub msg: &'a str,
}


impl<'a, T> Response<'a, T>
    where T: serde::Serialize {
    pub fn new(code: usize, data: T, msg: &'a str) -> Self {
        Response {
            code,
            data,
            msg,
        }
    }

    pub fn ok(data: T, msg: Option<&'a str>) -> Self {
        Response {
            code: 200,
            data,
            msg: msg.unwrap_or("操作成功"),
        }
    }

    pub fn fail(data: T, msg: Option<&'a str>) -> Self {
        Response {
            code: 300,
            data,
            msg: msg.unwrap_or("操作失败"),
        }
    }
}


pub trait ResponseBody {
    fn into_response(self) -> Result<String>;
}

impl<T> ResponseBody for Result<T>
    where T: Serialize
{
    fn into_response(self) -> Result<String> {
        match self {
            Ok(res) => {
                Ok(serde_json::to_string(&Response::ok(res, None))?)
            }
            Err(err) => {
                Ok(err.to_string())
            }
        }
    }
}

impl<'a, T> ResponseBody for Response<'a, T>
    where
        T: serde::ser::Serialize,
{
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

impl ResponseBody for () {
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok("", None))?)
    }
}

impl ResponseBody for bool {
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok(self, None))?)
    }
}

impl ResponseBody for usize {
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok(self, None))?)
    }
}

impl ResponseBody for isize {
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok(self, None))?)
    }
}

impl<T> ResponseBody for Vec<T>
    where
        T: serde::ser::Serialize,
{
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok(self, None))?)
    }
}

impl<K, V> ResponseBody for HashMap<K, V>
    where
        K: serde::ser::Serialize + Eq + std::hash::Hash,
        V: serde::ser::Serialize + Eq + std::hash::Hash,
{
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok(self, None))?)
    }
}

impl ResponseBody for String {
    fn into_response(self) -> Result<String> {
        Ok(Response::ok(self, None).into_response()?)
    }
}