# TransactionRequestNetworkFee

## Enum Variants

| Name | Description |
|---- | -----|
| String | For EVM-based blockchains only. The total transaction fee in the blockchain’s largest unit. Note: Only two of the three arguments can be specified in a single transaction: &#x60;gasLimit&#x60;, &#x60;gasPrice&#x60; and &#x60;networkFee&#x60;. Fireblocks recommends using a numeric string for accurate precision. Although a number input exists, it is deprecated. - The transaction blockchain fee. - For Ethereum, you can&#39;t pass gasPrice, gasLimit and networkFee all together. - A numeric value representation is required. |
| f64 | For EVM-based blockchains only. The total transaction fee in the blockchain’s largest unit. Note: Only two of the three arguments can be specified in a single transaction: &#x60;gasLimit&#x60;, &#x60;gasPrice&#x60; and &#x60;networkFee&#x60;. Fireblocks recommends using a numeric string for accurate precision. Although a number input exists, it is deprecated. - The transaction blockchain fee. - For Ethereum, you can&#39;t pass gasPrice, gasLimit and networkFee all together. - A numeric value representation is required. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


