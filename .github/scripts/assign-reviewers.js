import { Octokit } from "octokit";
import yaml from "js-yaml";
import fs from "fs";

const octokit = new Octokit({ auth: process.env.GITHUB_TOKEN });

const teams = yaml.load(fs.readFileSync('../teams.yml', 'utf8'));

const addTeam = (team, reviewers) => {
  teams[team].forEach(reviewer => {
    reviewers.add(reviewer.replace("@", ""));
  });
}

async function main() {
    const owner = "vlayer-xyz";
    const repo = "vlayer";

    // Fetch files changed in the PR
    const { data: files } = await octokit.rest.pulls.listFiles({
        owner,
        repo,
        pull_number: process.env.GITHUB_PULL_REQUEST_NUMBER,
    });

    // Criteria: Assign reviewers based on file paths
    const reviewers = new Set();

    const PATH_TO_TEAMS = {
      '.github/': ['Ops'],
      'ansible/': ['Ops'],
      'docker/': ['Ops'],
      'book/': ['DevEx'],
      'examples/': ['DevEx'],
      'packages/browser-extension': ['Web'],
      'rust/web_proof': ['Web'],
      'examples/simple_web_proof': ['Web'],
      'book/src/features/web.md': ['Web'],
      'rust/email_proof': ['Email'],
      'examples/simple_email_proof': ['Email'],
      'book/src/features/email.md': ['Email'],
      'book/src/features/json-and-regex.md': ['Email'],
      'rust/chain': ['Ethereum'],
      'rust/services': ['Ethereum'],
      'rust/mpt': ['Ethereum'],
      'rust/key_value': ['Ethereum'],
      'examples/simple_time_travel': ['Ethereum'],
      'examples/simple_teleport': ['Ethereum'],
    };

    files.forEach((file) => {
      const matchingPath = Object.keys(PATH_TO_TEAMS).find(path => 
        file.filename.startsWith(path)
      );
      
      if (matchingPath) {
        PATH_TO_TEAMS[matchingPath].forEach(team => addTeam(team, reviewers));
      }
    });

    // Output reviewers for the next step in the workflow
    const reviewersList = Array.from(reviewers);
    console.log({ reviewersList });
    await octokit.request('POST /repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers', {
      owner,
      repo,
      pull_number: process.env.GITHUB_PULL_REQUEST_NUMBER,
      reviewers: reviewersList,
      headers: {
        'X-GitHub-Api-Version': '2022-11-28'
      }
    })
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});
