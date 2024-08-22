#!/bin/bash

ABI_LOCATION="../{contracts,examples/**}/out/*.sol"
find ../{contracts,examples/**}/out/*.sol -name "*.json" -exec bash -c "echo export default \<const\> > {}.ts && cat {} >> {}.ts" \;
find ../{contracts,examples/**}/out/*.sol -name "*.json.ts" -exec bash -c 'mv $0 ${0%???????}ts' {} \;