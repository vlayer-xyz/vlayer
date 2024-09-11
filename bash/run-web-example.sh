VLAYER_HOME=$(git rev-parse --show-toplevel)

source ${VLAYER_HOME}/bash/run-services.sh &

cd ${VLAYER_HOME}/examples/web_proof/vlayer
bun run deploy.ts
bun run dev &

cd ${VLAYER_HOME}/packages/browser-plugin
bun run dev &