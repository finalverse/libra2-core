This is a small example of using the new `libra2` dependency. This shall be removed once we have
documentation/tests.

`pack2` contains a package which is used by `pack1` as follows:

```
[dependencies]
Pack2 = { libra2 = "http://localhost:8080", address = "default" }
```

To see it working:

```shell
# Start a node with an account
libra2 node run-local-testnet &
libra2 account create --account default --use-faucet 
# Compile and publish pack2
cd pack2
libra2 move compile --named-addresses project=default     
libra2 move publish --named-addresses project=default
# Compile pack1 agains the published pack2
cd ../pack1
libra2 move compile --named-addresses project=default     
```
