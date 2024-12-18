FROM oven/bun:latest

WORKDIR /app

COPY packages/test-json-server /app

RUN cat src/index.ts

RUN bun install

EXPOSE 3011

CMD ["bun", "run", "src/index.ts"]
