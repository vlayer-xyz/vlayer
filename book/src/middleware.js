import { next, rewrite } from '@vercel/edge';

const {
  NOTION_DATABASE_ID,
  NOTION_API_KEY,
  NOTION_API_VERSION,
  NOTION_API_URL,
  RESEND_API_KEY
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
  try {
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
    }
    await fetch(`${NOTION_API_URL}/pages/${row.id}`, options);
  } catch (err) {
    console.error("Notion update error: ", err.message);

  };
}

const deliverEmailNotification = async (login) => {
  try {
    await fetch('https://api.resend.com/emails', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${RESEND_API_KEY}`
      },
      body: JSON.stringify({
        "from": "vlayer Notifier <no_reply@vlayer.xyz>",
        "to": [
          "marek@vlayer.xyz", 
          "artur@vlayer.xyz"
        ],
        "subject": "New vlayer docs visit",
        "html": `${login} just visited the docs! 🎉 <br/><br/> See stats and manage access <a href="https://www.notion.so/vlayer/Docs-Allowlist-746a3a5f651d45c189717cb2ffb938fe>here</a>`
      })
    })
    console.log("Visit notification sent: ", login);
  } catch (err) {
    console.error("Email notification error: ", err.message);
  }
}

export default async function middleware(request) {
  const url = new URL(request.url);

  const staticFilesRegex = /\.(js|css|png|jpg|woff|woff2|svg|json|gif|mp4|ico)$/i;

  if (staticFilesRegex.test(url.pathname)) {
    console.log("Docs Auth path skipped: ", url.pathname)
    return next({
      headers: {
        'cache-control': 'public, max-age=31536000, immutable',
      },
    });
  }  

  try {
    const { login, password } = parseCreds(request.headers);
    console.log("Docs Auth Login attempt: ", [url.toString(), login]);

    const credsRow = await checkCreds(login, password);

    if(process.env.VERCEL_ENV === 'production') {
      await addCredsVisitLog(credsRow);
    }

    if(login.includes('@') && !login.includes('@vlayer.xyz')) {
      await deliverEmailNotification(login);
    }
    
    console.log("Docs Auth Login successful: ", [url.toString(), login]);

    return next();
  } catch (err) {
    console.log("Docs Auth Login issue: ", [url.toString(), err.message])
    url.pathname = '/api/auth';

    return rewrite(url);
  }
}