#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
example_name="simple_email_proof"

cd "${VLAYER_HOME}/examples/${example_name}"
mkdir -p .vercel
echo "{\"projectId\":\"${VERCEL_PROJECT_ID}\",\"orgId\":\"${VERCEL_ORG_ID}\"}" > .vercel/project.json

if [ "$VERCEL_ENV" == "production" ]; then
  DEPLOYMENT_URL=$(vercel --token "$VERCEL_TOKEN" --prod --yes --cwd ./vlayer/dist --scope "$VERCEL_ORG_ID" | tail -1)
  echo "simple-email-proof production deployment available at: $DEPLOYMENT_URL"
else
  DEPLOYMENT_URL=$(vercel --token $VERCEL_TOKEN --yes --cwd ./vlayer/dist --scope $VERCEL_ORG_ID | tail -1)
  COMMENT_BODY="The preview of the simple-email-proof example app is available at: $DEPLOYMENT_URL"
  curl -s -H "Authorization: token $GITHUB_TOKEN" \
    -X POST \
    -d "{\"body\":\"$COMMENT_BODY\"}" \
    "https://api.github.com/repos/${GITHUB_REPOSITORY}/issues/${PR_NUMBER}/comments"
fi
