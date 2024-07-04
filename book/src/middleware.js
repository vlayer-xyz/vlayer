import { next, rewrite } from '@vercel/edge';

export default function middleware(request) {
  const basicAuth = request.headers.get('authorization');

  const url = new URL(request.url);

  if (basicAuth) {
    const authValue = basicAuth.split(' ')[1]
    const [user, pwd] = atob(authValue).split(':')

    console.log("basic auth: ", [user, pwd]);

    if (user === 'admin' && pwd === 'admin') {
      console.log("going next...")
      return next();
    }
  }

  url.pathname = '/api/auth';
  console.log({ url })
  return rewrite(url);
}