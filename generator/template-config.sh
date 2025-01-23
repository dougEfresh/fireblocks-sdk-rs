#!/bin/bash
override=(xtra_parameters.rs)

for i in ${override[@]}; do
  echo -ne "  custom/models/$i:\n    templateType: SupportingFiles\n    destinationFilename: ${i}\n    folder: src/models\n"
done
