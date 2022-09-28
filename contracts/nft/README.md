# NFT

Arcnes NFT contract

This contract is extension of the token standard and covers features:
* Core logic [NEP-171](https://github.com/near/NEPs/blob/master/neps/nep-0171.md)
* Metadata [NEP-177](https://github.com/near/NEPs/blob/master/neps/nep-0177.md)
* Approvals [NEP-178](https://github.com/near/NEPs/blob/master/neps/nep-0178.md)
* Enumeration [NEP-181](https://github.com/near/NEPs/blob/master/neps/nep-0181.md)
* Royalties [NEP-199](https://github.com/near/NEPs/blob/master/neps/nep-0199.md)
* Metadata update by NFT contract owner
* Royalties update by NFT contract owner
* NFT mint by NFT contract owner
* NFT burn by NFT contract owner
* Locking mechanism for metadata and royalties update (only for NFT contract owner)

## Interface

### Calls

The testnet nft contract was deployed on `nft001.arcnes.testnet` account.

```
export NFT_CONTRACT_ID="nft001.arcnes.testnet"
export ACCOUNT_ID="your_account"
```

#### NFT default init
```
near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $ACCOUNT_ID
```

#### NFT mint

```
near call $NFT_CONTRACT_ID nft_mint '{"token_id": "token-1", "token_metadata": {"title": "My Non Fungible Team Token", "description": "The Team Most Certainly Goes :)", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "'$ACCOUNT_ID'"}' --accountId $ACCOUNT_ID --amount 0.1
```

### Views

#### NFT contract metadata

```
near view $NFT_CONTRACT_ID nft_metadata
```

#### Token details

```
near view $NFT_CONTRACT_ID nft_token '{"token_id": "token-1"}'
```
