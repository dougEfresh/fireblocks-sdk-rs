#!/bin/bash
override=(wallet_asset_external.rs wallet_asset.rs external_wallet_asset_detail.rs)

for i in  ${override[@]} ; do
  echo -ne "  custom/models/$i:\n    templateType: SupportingFiles\n    destinationFilename: ${i}\n    folder: src/models\n"; 
done
