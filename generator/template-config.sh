for i in  create_transaction_response.rs ; do
  echo -ne "  custom/models/$i.rs:\n    templateType: SupportingFiles\n    destinationFilename: $i\n    folder: src/models\n"; 
done
