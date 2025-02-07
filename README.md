# ace-pozk
ACE zk-shuffle for zypher mining network

- PoZK base sepolia prover: `0x40C2cAc8cD71FB82B2B3b72Ae24d797fE904FcE1`
- PoZK base sepolia verifier `0x1DD1253e7F245a763776a94A886FB3ED1FEed01b`

## Serialize (ABI encode/decode)
- Inputs (bytes)
```
"uint256"

uint256, // join_pk (aggregate_keys)
```

- Publics (bytes)
```
"uint256[], uint256"

uint256[], // deck serialize to u256 array
uint256,   // deck digest
```

- Proof (bytes)
```
"uint256[], uint256, uint256[], bytes"

uint256[], // new_deck serialize to uint256 array
uint256,   // new_deck digest
uint256[], // pkc
bytes,     // proof
```

## e.g. Hex (20 cards)
you can find test data in `./test` directory

## License

This project is licensed under [GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html).
