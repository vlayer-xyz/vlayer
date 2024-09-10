VLAYER_HOME=$(git rev-parse --show-toplevel)

cd ${VLAYER_HOME}/examples/web_proof/vlayer
bun run deploy.ts
bun run dev