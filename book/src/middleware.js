import { next, rewrite } from '@vercel/edge';

const {
  NOTION_DATABASE_ID,
  NOTION_API_KEY,
  NOTION_API_VERSION,
  NOTION_API_URL
} = process.env;

const NOTION_API_HEADERS = {
  'Content-Type': 'application/json',
  Authorization: `Bearer ${NOTION_API_KEY}`,
  'Notion-Version': NOTION_API_VERSION
}

// Control Docs access via Notion:
// https://www.notion.so/vlayer/Docs-Allowlist-746a3a5f651d45c189717cb2ffb938fe

const parseCreds = (headers) => {
  const basicAuth = headers.get('authorization');

  if (basicAuth) {
    const authValue = basicAuth.split(' ')[1]
    const [login, password] = atob(authValue).split(':')

    if(!login || !password) {
      throw new Error("Docs Auth error: Login or password not provided");
    }

    return { login, password };
  } else {
    throw new Error("Docs Auth error: No HTTP credentials provided");
  
  }
}

const checkCreds = async (login, password) => {
  const options = {
    method: 'POST',
    headers: NOTION_API_HEADERS,
    body: JSON.stringify({
      filter: {
        and: [
          {
            property: "Login",
            title: {
              equals: login
            }
          },
          {
            property: "Password",
            rich_text: {
              equals: password
            }
          }
        ]
      }
    })
  };
  
  const response = await fetch(`${NOTION_API_URL}/databases/${NOTION_DATABASE_ID}/query`, options);
  const parsed = await response.json();
  
  if (!parsed.results.length) {
    throw new Error("auth failed");
  }

  return parsed.results[0];
}

const addCredsVisitLog = async (row) => {
  const currentLoginCount = row.properties["Login Count"].number || 0;

  const options = {
    method: 'PATCH',
    headers: NOTION_API_HEADERS,
    body: JSON.stringify({
      properties: {
        "Last Access At": {
          date: {
            start: new Date().toISOString(),
          }
        },
        "Login Count": {
          number: currentLoginCount + 1
        }
      }
    })
  };
  

  await fetch(`${NOTION_API_URL}/pages/${row.id}`, options);
}

export default async function middleware(request) {
  const url = new URL(request.url);

  try {
    const { login, password } = parseCreds(request.headers);
    console.log("Docs Auth Login attempt: ", [url.toString(), login]);

    const credsRow = await checkCreds(login, password);
    await addCredsVisitLog(credsRow);
    console.log("Docs Auth Login successful: ", [url.toString(), login]);

    return next();
  } catch (err) {
    console.log("Docs Auth Login issue: ", [url.toString(), err.message])
    url.pathname = '/api/auth';

    return rewrite(url);
  }
}