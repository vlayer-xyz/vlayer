export async function middleware(req) {
    const basicAuth = req.headers.get('authorization');

    if (basicAuth) {
        const auth = basicAuth.split(' ')[1];
        const [user, pwd] = Buffer.from(auth, 'base64').toString().split(':');

        // Replace 'username' and 'password' with your actual credentials
        if (user === 'vlayer' && pwd === 'layerv') {
            return new Response('Authorized', {
                status: 200,
            });
        }
    }

    return new Response('Unauthorized', {
        status: 401,
        headers: {
            'WWW-Authenticate': 'Basic realm="Secure Area"',
        },
    });
}