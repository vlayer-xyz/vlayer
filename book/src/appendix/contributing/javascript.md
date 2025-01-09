# Contributing to vlayer JavaScript codebase

## Prerequisites

To start working with this repository, you will need to install following software:
- [Bun](https://bun.sh/) JavaScript runtime. 

## Bumping version 
1. Apply changes to the code
1. Run `bun changeset`
1. Submit information about your changes (would be visible in the changelog)
1. Run `bun changeset version`
1. Commit modified files changes 
1. Push

Quick list of common questions to get you started engaging with changesets (tool for versioning) is in [their docs](https://github.com/changesets/changesets/blob/main/docs/common-questions.md)

## Troubleshooting

### Hanging SDK tests

If you see the following when trying to run SDK unit tests

```sh
$ cd packages/sdk
$ bun run test:unit
 vitest --run

 RUN  v2.1.4 /Users/kubkon/dev/vlayer/vlayer/packages/sdk
```

and nothing happening for a longer while, make sure you have [Node.js](https://nodejs.org) installed.
