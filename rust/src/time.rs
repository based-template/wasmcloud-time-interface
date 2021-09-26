// This file is generated automatically using wasmcloud-weld and smithy model definitions
//

#![allow(clippy::ptr_arg)]
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Cow, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

/// wrapper for timestamp + string representing RFC to format time-string
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FormatTimeRequest {
    #[serde(default)]
    pub rfc: String,
    pub timestamp: u64,
}

/// The Time service has two methods, GetTimestamp and FormatTimestamp, which
/// return the current time as a U64 Unix epoch timestamp and a formatted string of a timestamp, respectively
/// wasmbus.contractId: auxiliary::interfaces::time
/// wasmbus.providerReceive
#[async_trait]
pub trait Time {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "auxiliary::interfaces::time"
    }
    /// Provides current time according to Unix epoch format with millisecond resolution
    async fn get_timestamp(&self, ctx: &Context) -> RpcResult<u64>;
    /// takes structure containing U64 Unix epoch timestamp + RFC string,
    /// returns formatted string representing timestamp according to RFC (eg: RFC2822, RFC3339)
    async fn format_timestamp(&self, ctx: &Context, arg: &FormatTimeRequest) -> RpcResult<String>;
}

/// TimeReceiver receives messages defined in the Time service trait
/// The Time service has two methods, GetTimestamp and FormatTimestamp, which
/// return the current time as a U64 Unix epoch timestamp and a formatted string of a timestamp, respectively
#[doc(hidden)]
#[async_trait]
pub trait TimeReceiver: MessageDispatch + Time {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "GetTimestamp" => {
                let resp = Time::get_timestamp(self, ctx).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Time.GetTimestamp",
                    arg: buf,
                })
            }
            "FormatTimestamp" => {
                let value: FormatTimeRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Time::format_timestamp(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Time.FormatTimestamp",
                    arg: buf,
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Time::{}",
                message.method
            ))),
        }
    }
}

/// TimeSender sends messages to a Time service
/// The Time service has two methods, GetTimestamp and FormatTimestamp, which
/// return the current time as a U64 Unix epoch timestamp and a formatted string of a timestamp, respectively
/// client for sending Time messages
#[derive(Debug)]
pub struct TimeSender<T: Transport> {
    transport: T,
}

impl<T: Transport> TimeSender<T> {
    /// Constructs a TimeSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl TimeSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Time provider
    /// implementing the 'auxiliary::interfaces::time' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "auxiliary::interfaces::time",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Time provider
    /// implementing the 'auxiliary::interfaces::time' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "auxiliary::interfaces::time",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Time for TimeSender<T> {
    #[allow(unused)]
    /// Provides current time according to Unix epoch format with millisecond resolution
    async fn get_timestamp(&self, ctx: &Context) -> RpcResult<u64> {
        let arg = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Time.GetTimestamp",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "GetTimestamp", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// takes structure containing U64 Unix epoch timestamp + RFC string,
    /// returns formatted string representing timestamp according to RFC (eg: RFC2822, RFC3339)
    async fn format_timestamp(&self, ctx: &Context, arg: &FormatTimeRequest) -> RpcResult<String> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Time.FormatTimestamp",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "FormatTimestamp", e)))?;
        Ok(value)
    }
}
