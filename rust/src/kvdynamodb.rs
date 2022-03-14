// This file is generated automatically using wasmcloud/weld-codegen 0.4.2

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

pub const SMITHY_VERSION: &str = "1.0";

/// Response to get request
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetResponse {
    /// the value, if it existed
    #[serde(default)]
    pub value: String,
    /// whether or not the value existed
    #[serde(default)]
    pub exists: bool,
}

// Encode GetResponse as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_get_response<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &GetResponse,
) -> RpcResult<()> {
    e.array(2)?;
    e.str(&val.value)?;
    e.bool(val.exists)?;
    Ok(())
}

// Decode GetResponse from cbor input stream
#[doc(hidden)]
pub fn decode_get_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<GetResponse, RpcError> {
    let __result = {
        let mut value: Option<String> = None;
        let mut exists: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct GetResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct GetResponse: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => value = Some(d.str()?.to_string()),
                    1 => exists = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct GetResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "value" => value = Some(d.str()?.to_string()),
                    "exists" => exists = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        GetResponse {
            value: if let Some(__x) = value {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field GetResponse.value (#0)".to_string(),
                ));
            },

            exists: if let Some(__x) = exists {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field GetResponse.exists (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct KeysRequest {
    /// search for only keys that match a particular glob expression
    #[serde(rename = "globExpression")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glob_expression: Option<String>,
    /// optional configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

// Encode KeysRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_keys_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &KeysRequest,
) -> RpcResult<()> {
    e.array(2)?;
    if let Some(val) = val.glob_expression.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.config.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode KeysRequest from cbor input stream
#[doc(hidden)]
pub fn decode_keys_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<KeysRequest, RpcError> {
    let __result = {
        let mut glob_expression: Option<Option<String>> = Some(None);
        let mut config: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct KeysRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct KeysRequest: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        glob_expression = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    1 => {
                        config = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct KeysRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "globExpression" => {
                        glob_expression = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "config" => {
                        config = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        KeysRequest {
            glob_expression: glob_expression.unwrap(),
            config: config.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetRequest {
    /// the key name to change (or create)
    #[serde(default)]
    pub key: String,
    /// the new value
    #[serde(default)]
    pub value: String,
    /// expiration time in seconds 0 for no expiration
    #[serde(default)]
    pub expires: u32,
}

// Encode SetRequest as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_set_request<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &SetRequest,
) -> RpcResult<()> {
    e.array(3)?;
    e.str(&val.key)?;
    e.str(&val.value)?;
    e.u32(val.expires)?;
    Ok(())
}

// Decode SetRequest from cbor input stream
#[doc(hidden)]
pub fn decode_set_request(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<SetRequest, RpcError> {
    let __result = {
        let mut key: Option<String> = None;
        let mut value: Option<String> = None;
        let mut expires: Option<u32> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct SetRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct SetRequest: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => key = Some(d.str()?.to_string()),
                    1 => value = Some(d.str()?.to_string()),
                    2 => expires = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct SetRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "key" => key = Some(d.str()?.to_string()),
                    "value" => value = Some(d.str()?.to_string()),
                    "expires" => expires = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        }
        SetRequest {
            key: if let Some(__x) = key {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field SetRequest.key (#0)".to_string(),
                ));
            },

            value: if let Some(__x) = value {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field SetRequest.value (#1)".to_string(),
                ));
            },

            expires: if let Some(__x) = expires {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field SetRequest.expires (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// list of strings
pub type StringList = Vec<String>;

// Encode StringList as CBOR and append to output stream
#[doc(hidden)]
pub fn encode_string_list<W: wasmbus_rpc::cbor::Write>(
    e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StringList,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        e.str(item)?;
    }
    Ok(())
}

// Decode StringList from cbor input stream
#[doc(hidden)]
pub fn decode_string_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<StringList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<String> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(d.str()?.to_string())
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<String> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(d.str()?.to_string()),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// The Dynamodb service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
/// wasmbus.contractId: aws:kvdynamodb
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait KvDynamoDb {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "aws:kvdynamodb"
    }
    /// Gets a value for a specified key. If the key exists,
    /// the return structure contains exists: true and the value,
    /// otherwise the return structure contains exists == false.
    async fn get<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<GetResponse>;
    /// Sets the value of a key.
    /// expires is an optional number of seconds before the value should be automatically deleted,
    /// or 0 for no expiration.
    async fn set(&self, ctx: &Context, arg: &SetRequest) -> RpcResult<()>;
    /// Deletes a key, returning true if the key was deleted
    async fn del<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool>;
    /// fetches a list of keys present in the kv store
    async fn keys(&self, ctx: &Context, arg: &KeysRequest) -> RpcResult<StringList>;
}

/// KvDynamoDbReceiver receives messages defined in the KvDynamoDb service trait
/// The Dynamodb service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
#[doc(hidden)]
#[async_trait]
pub trait KvDynamoDbReceiver: MessageDispatch + KvDynamoDb {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Get" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;
                let resp = KvDynamoDb::get(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "KvDynamoDb.Get",
                    arg: Cow::Owned(buf),
                })
            }
            "Set" => {
                let value: SetRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'SetRequest': {}", e)))?;
                let _resp = KvDynamoDb::set(self, ctx, &value).await?;
                let buf = Vec::new();
                Ok(Message {
                    method: "KvDynamoDb.Set",
                    arg: Cow::Owned(buf),
                })
            }
            "Del" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;
                let resp = KvDynamoDb::del(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "KvDynamoDb.Del",
                    arg: Cow::Owned(buf),
                })
            }
            "Keys" => {
                let value: KeysRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'KeysRequest': {}", e)))?;
                let resp = KvDynamoDb::keys(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "KvDynamoDb.Keys",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "KvDynamoDb::{}",
                message.method
            ))),
        }
    }
}

/// KvDynamoDbSender sends messages to a KvDynamoDb service
/// The Dynamodb service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
/// client for sending KvDynamoDb messages
#[derive(Debug)]
pub struct KvDynamoDbSender<T: Transport> {
    transport: T,
}

impl<T: Transport> KvDynamoDbSender<T> {
    /// Constructs a KvDynamoDbSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> KvDynamoDbSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl KvDynamoDbSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl KvDynamoDbSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a KvDynamoDb provider
    /// implementing the 'aws:kvdynamodb' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("aws:kvdynamodb", "default")
                .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a KvDynamoDb provider
    /// implementing the 'aws:kvdynamodb' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("aws:kvdynamodb", link_name)?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> KvDynamoDb for KvDynamoDbSender<T> {
    #[allow(unused)]
    /// Gets a value for a specified key. If the key exists,
    /// the return structure contains exists: true and the value,
    /// otherwise the return structure contains exists == false.
    async fn get<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<GetResponse> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "KvDynamoDb.Get",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: GetResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': GetResponse", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Sets the value of a key.
    /// expires is an optional number of seconds before the value should be automatically deleted,
    /// or 0 for no expiration.
    async fn set(&self, ctx: &Context, arg: &SetRequest) -> RpcResult<()> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "KvDynamoDb.Set",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
    #[allow(unused)]
    /// Deletes a key, returning true if the key was deleted
    async fn del<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "KvDynamoDb.Del",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: bool = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Boolean", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// fetches a list of keys present in the kv store
    async fn keys(&self, ctx: &Context, arg: &KeysRequest) -> RpcResult<StringList> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "KvDynamoDb.Keys",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: StringList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': StringList", e)))?;
        Ok(value)
    }
}
