import { next, rewrite } from '@vercel/edge';

export default function middleware(request) {
  const basicAuth = request.headers.get('authorization');

  const url = new URL(request.url);

  if (basicAuth) {
    const authValue = basicAuth.split(' ')[1]
    const [user, pwd] = atob(authValue).split(':')

    console.log("Login attempt: ", [url.toString(), user]);

    if (user === 'admin' && pwd === 'admin') {
      console.log("Login successful: ", [url.toString(), user]);
      return next();
    }
  }

  console.log("No HTTP auth: ", [url.toString()])
  url.pathname = '/api/auth';

  return rewrite(url);
}