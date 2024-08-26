#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

for output_json in $(find "${VLAYER_HOME}" -wholename "*/out/*.sol/*.json") ; do 
  output_ts="${output_json%.json}.ts"

  echo "Generating ${output_ts}" 
  
  echo "export default <const>" >"${output_ts}" 
  cat "${output_json}" | jq >>"${output_ts}"
 
done
