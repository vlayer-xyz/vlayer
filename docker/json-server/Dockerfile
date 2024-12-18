FROM oven/bun:latest

WORKDIR /app

COPY packages/test-json-server /app

RUN bun install

EXPOSE 3011

CMD ["bun", "run", "src/index.ts"]
