# TravelRuleOwnershipProof

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of ownership proof. Example values: - `eip-191`: Ethereum signature proof - `eip-712`: Ethereum typed data signature proof - `bip-137`: Bitcoin signature proof - `microtransfer`: Microtransaction (Satoshi test) - `screenshot`: Uploaded screenshot of the wallet - `self-declaration`: Checkbox attestation of ownership | [optional]
**proof** | Option<**String**> | The cryptographic signature, transaction hash, or other proof depending on the type. Examples: - For `eip-191`: `0x3dd4a17a...ce4a2bcd1b` - For `microtransfer`: The transaction hash `H3V8GXBy39Dz...tr3TSTkY=` | [optional]
**attestation** | Option<**String**> | A human-readable statement of wallet ownership. Required for signature proofs and self-declarations. Examples: - `I certify that ETH account 0x896B...0b9b belongs to me.` - `I hereby declare that the blockchain address 0xa437bEed902AF9338B7DEB23848e195d85019510 is under my control.` | [optional]
**address** | Option<**String**> | The wallet address being verified. Examples: - For Ethereum: `0x896B...0b9b` - For Bitcoin: `1442...dxhsQ` | [optional]
**wallet_provider** | Option<**String**> | The wallet provider or method used for verification. Examples: - For Metamask: `Metamask` - For manual signature: `manual` | [optional]
**url** | Option<**String**> | The URL for the uploaded screenshot (for `screenshot` proof types only). Example: `https://example.com/uploaded_image.png` | [optional]
**confirmed** | Option<**bool**> | Whether the user confirmed ownership of the wallet (for `self-declaration` proofs). Example: `true` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


