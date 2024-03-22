# Build contract
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

# Create SubAccount
We are going to create a subaccount for deploying the contract and adding initial balance for contract
```
near create-account sub-acct.example-acct.testnet --useAccount example-acct.testnet --initialBalance 5
```
near create-account nearsports.testenoid.testnet --useAccount testenoid.testnet --initialBalance 5
# Deployment
```
near deploy sub-acct.example-acct.testnet target/wasm32-unknown-unknown/release/contract.wasm
near deploy nearsports.testenoid.testnet target/wasm32-unknown-unknown/release/neararena.wasm
```

# Initialization
```

near call sub-acct.example-acct.testnet '{"owner_id": "example-acct.testnet"}' init --accountId=example-acct.testnet 

near call nearsports.testenoid.testnet init '{"owner_id": "testenoid.testnet"}' --accountId=testenoid.testnet

```


# Calling a Change Function
near call nearsports.testenoid.testnet create_tournament '{"tournament_id":"1", "name":"lolito", "prize_pool":"1000000000000000000000000"}' --accountId=testenoid.testnet

near call nearsports.testenoid.testnet  create_tournament '{"tournament_id": "torneo123", "name": "Torneo Épico", "prize_pool": "1000000000000000000000000", "img_url":"https://i.pinimg.com/736x/6d/3b/cb/6d3bcbd544aaf23fd6f7b5362330775a.jpg"}' --accountId=testenoid.testnet --amount 1


Calling a View Function
// get_product
near view contractor.testenoid.testnet get_product '{"id": "1"}'





Calling a View Function on Product Model
near view contractor.testenoid.testnet get_product '{"id": "1"}'




near call nearsports.testenoid.testnet create_tournament '{"tournament_id": "tour1", "name": "Torneo de Prueba", "prize_pool": "1000000000000000000000000"}' --accountId=testenoid.testnet

near call nearsports.testenoid.testnet create_tournament '{"tournament_id": "tour1", "name": "Torneo de Prueba", "prize_pool": "1000000000000000000000000"}' --accountId=testenoid.testnet
