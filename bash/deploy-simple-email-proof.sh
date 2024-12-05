#!/usr/bin/env bash

set -ueo pipefail

echo '::group::foundry installation'
curl -L https://foundry.paradigm.xyz | bash
export PATH="$PATH:$HOME/.config/.foundry/bin"
foundryup
echo '::endgroup::'

echo '::group::vlayer installation'
curl -SL https://install.vlayer.xyz | bash
export PATH="$PATH:$HOME/.config/.vlayer/bin"
vlayerup
echo '::endgroup::'

echo '::group::bun installation'
curl -fsSL https://bun.sh/install | bash
export PATH="$PATH:~/.bun/bin"
echo '::endgroup::'

git config --global user.email "test@example.com"
git config --global user.name "Github Runner"

VLAYER_HOME=$(git rev-parse --show-toplevel)
example_name="simple-email-proof"

echo "::group::Initializing vlayer template: ${example_name}"
VLAYER_TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX-)
cd ${VLAYER_TEMP_DIR}
vlayer init --template "${example_name}"
forge build
cd vlayer
bun install
echo '::endgroup::'

echo "::group::vlayer run deploy:testnet: ${example_name}"
bun run deploy:testnet
echo '::endgroup::'

echo "::group::vlayer install vercel"
cd ..
npm install -g vercel
echo '::endgroup::'

echo "::group::vlayer deploy to vercel: ${example_name}"
mkdir -p .vercel
echo "{\"projectId\":\"${VERCEL_PROJECT_ID}\",\"orgId\":\"${VERCEL_ORG_ID}\"}" > .vercel/project.json

if [ "$VERCEL_ENV" == "production" ]; then
  vercel env pull ./vlayer/.env.testnet --token "$VERCEL_TOKEN" --prod
  vercel env pull ./vlayer/.env.testnet.local --token "$VERCEL_TOKEN"
  vercel --token "$VERCEL_TOKEN" --prod --yes --cwd ./ --scope "$VERCEL_ORG_ID" | tail -1
  # echo "Book production deployment available at: $DEPLOYMENT_URL"
else
  vercel env pull ./vlayer/.env.testnet --token "$VERCEL_TOKEN"
  vercel env pull ./vlayer/.env.testnet.local --token "$VERCEL_TOKEN"
  vercel --token "$VERCEL_TOKEN" --yes --cwd ./ --scope "$VERCEL_ORG_ID" | tail -1
  # DEPLOYMENT_URL=$(vercel --token $VERCEL_TOKEN --yes --cwd ./book --scope $VERCEL_ORG_ID | tail -1)
  # COMMENT_BODY="The preview of the simple-email-proof example app is available at: $DEPLOYMENT_URL"
  # curl -s -H "Authorization: token $GITHUB_TOKEN" \
  #   -X POST \
  #   -d "{\"body\":\"$COMMENT_BODY\"}" \
  #   "https://api.github.com/repos/${{ github.repository }}/issues/$PR_NUMBER/comments"
fi
echo '::endgroup::'
