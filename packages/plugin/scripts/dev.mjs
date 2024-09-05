try {
  await $`tmux kill-session -t "vlayer-plugin"`;
  // eslint-disable-next-line no-unused-vars, @typescript-eslint/no-unused-vars
} catch (err) {
  console.log("vlayer-plugin is not running. Nothing to kill");
}
await $`tmux new-session -d -s "vlayer-plugin" `;

await $`tmux new-window -n "websocketproxy"`;
await $`tmux send-keys -t websocketproxy "cd websockify && docker run -it --rm -p 55688:80 novnc/websockify 80 rickandmortyapi.com:443
  " C-m`;
await $`tmux new-window -n "tlsn"`;
await $`tmux send-keys -t tlsn "cd tlsn/notary-server && cargo run --release" C-m`;

await $`tmux new-window -n "anvil"`;
await $`tmux send-keys -t anvil "anvil" C-m`;

await $`tmux new-window -n "deployContracts"`;
await $`tmux send-keys -t deployContracts "bun ./scripts/deployContracts.js " C-m`;

await $`tmux new-window -n "webapp"`;
await $`tmux send-keys -t webapp "cd webapp && bun run dev" C-m`;

await $`tmux new-window -n "plugin"`;
await $`tmux send-keys -t plugin "cd browser-plugin && bun run dev" C-m`;

await $`tmux new-window -n "vlayer"`;
await $`tmux send-keys -t vlayer "vlayer serve" C-m`;

await $`tmux attach-session -d`;
