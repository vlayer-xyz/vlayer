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
    const vlayerClient = createVlayerClient({ token: process.env.VLAYER_API_TOKEN });
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
export VLAYER_API_TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJob3N0IjoiYXBpLnguY29tIiwicG9ydCI6NDQzLCJleHAiOjE4NDQ2NzQ0MDczNzA5NTUxNjE1LCJzdWIiOiJ0ZXN0In0.UMwVhJ48C58t8dwrXKE8yE0La6n8ibkwnylOaQiNtOerioNClCjmeTg0CXomv3hao0LRPHTV5iH0fhTGCP1oT_ZMt7oHJYKqaA1NkJMjpoNt5p9yUr9GHhbhM3GJnBNjG6iM5kG_Ov4qQyrNL3TbNm5oQCv0MNA9b-9bn9WroHTAzeMFvJZp0bS65pUX3zESkx3dc3lkgSpsXW69aou0sHOF5m7Ap6k0z0VniWuFev5WE8JrWSnT76-_hIvudrykyFBOm-Uj4mGvbJBG0WoriEOCoe8n5RDZniDdZ1hjO5H5fb9cvfk1Lec0fHHW37ECfcxuImxpcozr44vHZdlPO4MwUiloKrRDiw-aywQtpwd3PLH0vme73dH07OeXO_mTWRmaciMoXHLCDntcn3N2oVYWGIdgpCXEFztFB6yw5a19_KwzBbapxzQ9ZC3MIllsVmUDcVh5ZBmTCFNYlFIH6zzTau6bee3fSTlsbIt1fwIxV5Tn3odXrOH59zUHyWplCExizzgHFFERsO_AlSeYnJrp0lE6bwtJlAOHa-AvVbWQyNXhZ8kV6hebwOhFcWd8i7cEGR6MlMOeC1LCOg9s0mafv6G8hZTnlenI85ohRyKnA_ciXMxLi3iYFqJ9UGLffzhumzndYjnF1qGv8OsNdB07b6WWSlrJsmjIQ4yBr7k
node index
echo "::endgroup::"
