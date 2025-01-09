for i in  wallet_asset_external.rs ; do
  echo -ne "  custom/models/$i.rs:\n    templateType: SupportingFiles\n    destinationFilename: $i\n    folder: src/models\n"; 
done
