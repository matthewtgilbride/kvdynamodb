# [wasmcloud](https://wasmcloud.com/) interface for AWS DynamoDB

Provides a simple key/value [interface](https://wasmcloud.dev/interfaces/) to an AWS Dynamo table.

The contract is more or less a subset of the first party
[keyvalue](https://github.com/wasmCloud/interfaces/tree/main/keyvalue) interface.  Supporting the following
identical operations:
 - `get` a value by its key
 - `set` a key/value pair
 - `del` remove a key/value pair

In addition, this interface exposes a `keys` operation, that allows iterating over all keys with a cursor.
