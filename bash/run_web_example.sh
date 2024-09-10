VLAYER_HOME=$(git rev-parse --show-toplevel)

source ${VLAYER_HOME}/bash/run-services.sh &
source ${VLAYER_HOME}/bash/run-web-app.sh &
source ${VLAYER_HOME}/bash/run-web-extension.sh &

