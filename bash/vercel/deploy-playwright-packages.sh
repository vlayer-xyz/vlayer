#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

echo "Deploying playwright report for packages"
cd "${VLAYER_HOME}/packages/playwright-report/"
mkdir -p .vercel
echo "{
    \"projectId\":\"${VERCEL_PROJECT_ID}\",
    \"orgId\":\"${VERCEL_ORG_ID}\",
    \"builds\": [
        {
          \"src\": \"index.html\",
          \"use\": \"@vercel/static\"
        }
      ]
}" > .vercel/project.json

DEPLOYMENT_URL=$(vercel --token $VERCEL_TOKEN )
if [ "$POST_COMMENT" == "true" ]; then
  COMMENT_BODY="The playwright report is available at: $DEPLOYMENT_URL \n\n Context: $CONTEXT"
  curl -s -H "Authorization: token $GITHUB_TOKEN" \
    -X POST \
    -d "{\"body\":\"$COMMENT_BODY\"}" \
    "https://api.github.com/repos/${GITHUB_REPOSITORY}/issues/${PR_NUMBER}/comments"
fi

echo "deployment_url=${DEPLOYMENT_URL}" >> $GITHUB_OUTPUT
