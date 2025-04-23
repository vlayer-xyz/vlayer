const http = require('http');
const url = require('url');

const server = http.createServer((req, res) => {
  if (req.method === 'GET') {
    const parsedUrl = url.parse(req.url, true);
    const token = parsedUrl.query.token;

    console.log(`Received new request: ?token=${token}`);

    if (token) {
      const [host, portStr] = token.split(':');
      const port = parseInt(portStr, 10) || 443;

      if (!host) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Invalid token format. Expected host:port' }));
        return;
      }

      const response = {
        host,
        port
      };

      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify(response));
    } else {
      res.writeHead(400, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({ error: 'Missing token parameter' }));
    }
  } else {
    res.writeHead(405, { 'Content-Type': 'text/plain' });
    res.end('405 Method Not Allowed\n');
  }
});

const PORT = parseInt(process.env.PORT, 10) || 3010;

server.listen(PORT, '0.0.0.0', () => {
  console.log(`Server is running at http://localhost:${PORT}`);
});

