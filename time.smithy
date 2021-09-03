// time.smithy
// A simple service that calculates the factorial of a whole number


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.auxiliary.interfaces.time", crate: "time_interface" } ]

namespace org.auxiliary.interfaces.time

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The Time service has two methods, GetTimestamp and FormatTimestamp, which
/// return the current time as a U64 Unix epoch timestamp and a formatted string of a timestamp, respectively
@wasmbus(
    contractId: "auxiliary::interfaces::time",
    providerReceive: true )
service Time {
  version: "0.1",
  operations: [ GetTimestamp, FormatTimestamp ]
}

/// Provides current time according to Unix epoch format with millisecond resolution
operation GetTimestamp {
  output: U64
}

/// takes structure containing U64 Unix epoch timestamp + RFC string,
/// returns formatted string representing timestamp according to RFC (eg: RFC2822, RFC3339)
operation FormatTimestamp {
    input: FormatTimeRequest,
    output: String
}

/// wrapper for timestamp + string representing RFC to format time-string
structure FormatTimeRequest {
    @required
    timestamp: U64,

    @required
    rfc: String,
}
