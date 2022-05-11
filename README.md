# casper-key-manager
Key manager extensions

## Deploy
```
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://host:7777/ \
    --secret-key ./secret_key.pem \
    --session-path ./contract/target/wasm32-unknown-unknown/release/contract.wasm \
    --payment-amount 20000000000
```

## Call deployment

### Add a new key by account-hash
```
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://host:7777/ \
    --secret-key ./secret_key.pem \
    --payment-amount 20000000000 \
    --session-name keys_manager_ext \
    --session-entry-point "set_key_weight" \
    --session-arg="account_hash:account_hash='account-hash-xxx'" \
    --session-arg="weight:u8='1'"
```

### Remove a key by account-hash
```
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://host:7777/ \
    --secret-key ./secret_key.pem \
    --payment-amount 20000000000 \
    --session-name keys_manager_ext \
    --session-entry-point "set_key_weight" \
    --session-arg="account_hash:account_hash='account-hash-xxx'" \
    --session-arg="weight:u8='0'"
```