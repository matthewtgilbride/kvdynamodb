// kvdynamodb.smithy
// A bare bones key value interface for AWS DynamoDB


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "com.mattgilbride.kvdynamodb", crate: "wasmcloud_interface_kvdynamodb" } ]

namespace com.mattgilbride.kvdynamodb

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#n
use org.wasmcloud.model#U32
use org.wasmcloud.model#I32

/// The Dynamodb service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
@wasmbus(
    contractId: "aws:kvdynamodb",
    actorReceive: true,
    providerReceive: true )
service KvDynamoDb {
  version: "0.1",
  operations: [ Get, Set, Del, Keys ]
}

/// Gets a value for a specified key. If the key exists,
/// the return structure contains exists: true and the value,
/// otherwise the return structure contains exists == false.
@readonly
operation Get {
  input: String,
  output: GetResponse,
}

/// Response to get request
structure GetResponse {
    /// the value, if it existed
    @required
    @n(0)
    value: String,
    /// whether or not the value existed
    @required
    @n(1)
    exists: Boolean,
}

/// Sets the value of a key.
/// expires is an optional number of seconds before the value should be automatically deleted,
/// or 0 for no expiration.
operation Set {
  input: SetRequest,
}

structure SetRequest {
    /// the key name to change (or create)
    @required
    @n(0)
    key: String,

    /// the new value
    @required
    @n(1)
    value: String,

    /// expiration time in seconds 0 for no expiration
    @required
    @n(2)
    expires: U32,
}

/// Deletes a key, returning true if the key was deleted
operation Del {
  input: String,
  output: Boolean,
}

/// list of strings
list StringList {
  member: String
}

/// fetches a list of keys present in the kv store
@readonly
operation Keys {
  input: KeysRequest,
  output: KeysResponse,
}

structure KeysRequest {
  /// pointer to the next cursor from a paginated response
  @n(0)
  cursor: String
}

structure KeysResponse {
  @n(0)
  @required
  keys: StringList
  @n(1)
  cursor: String
}
