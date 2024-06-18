export async function middleware(req) {
    console.log("*****");
    const basicAuth = req.headers.get('authorization');
    console.log(basicAuth);

    if (basicAuth) {
        const auth = basicAuth.split(' ')[1];
        const [user, pwd] = Buffer.from(auth, 'base64').toString().split(':');

        if (user === 'vlayer' && pwd === 'czeczota29') {
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