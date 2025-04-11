#!/usr/bin/env bash

set -ueo pipefail

echo "::group::creating js project"
mkdir ${HOME}/js_project_test
cd ${HOME}/js_project_test
echo "{
  \"type\": \"module\"
}" >> package.json
echo "::endgroup::"

echo "::group::vlayer sdk installation"
npm install @vlayer/sdk
echo "::endgroup::"

echo "::group::creating simple code using sdk"

echo "import { createVlayerClient } from '@vlayer/sdk';
import { strict as assert } from 'assert';

try {
    const vlayerClient = createVlayerClient();
    const hash = await vlayerClient.prove({
        proverAbi: [{
            'type': 'function',
            'name': 'main',
        }],
    });
    await vlayerClient.waitForProvingResult({ hash });
    throw Error('Exception expected');
} catch (error) {
    assert.equal(error.message, 'Error response: Invalid params');
    console.log('Vlayer server called');
}
" >> index.js

echo "::endgroup::"

echo "::group::running code"
node index
echo "::endgroup::"
