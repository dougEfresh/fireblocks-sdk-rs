for i in paginated_address_response.rs vault_wallet_address.rs; do  echo -ne "  custom/models/$i.rs:\n    templateType: SupportingFiles\n    destinationFilename: $i\n    folder: src/models\n"; done
