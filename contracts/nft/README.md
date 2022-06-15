# NFT

ArtLake NFT contract

## Interface

### Calls

The nft contract was deployed on `nft_test.artlake.testnet` account.

```
export NFT_CONTRACT_ID="nft_test.artlake.testnet"
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
