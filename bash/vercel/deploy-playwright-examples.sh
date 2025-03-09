#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/../lib/examples.sh"

for example in $(get_examples); do
  echo "Deploying playwright report for ${example}"
  if [ -d "${VLAYER_HOME}${EXAMPLES_PATH}${example}/vlayer/playwright-report/" ]; then
    cd "${VLAYER_HOME}${EXAMPLES_PATH}${example}/vlayer/playwright-report/"
  else
    echo "No playwright report found for ${example}. Skipping..."
    continue
  fi
  mkdir -p .vercel
  echo "VERCEL_PROJECT_ID: $VERCEL_PROJECT_ID"
  echo "VERCEL_ORG_ID: $VERCEL_ORG_ID"
  echo "VERCEL_TOKEN: $VERCEL_TOKEN"
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
  echo "DEPLOYMENT_URL: $DEPLOYMENT_URL"
  COMMENT_BODY="The playwright report of the ${example} example app is available at: $DEPLOYMENT_URL"
  curl -s -H "Authorization: token $GITHUB_TOKEN" \
    -X POST \
    -d "{\"body\":\"$COMMENT_BODY\"}" \
    "https://api.github.com/repos/${GITHUB_REPOSITORY}/issues/${PR_NUMBER}/comments"

  echo "deployment_url=${DEPLOYMENT_URL}" >> $GITHUB_OUTPUT
done
