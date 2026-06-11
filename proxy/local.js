// Servidor proxy local para desarrollo
// Ejecutar: node proxy/local.js

const http = require('http');
const https = require('https');
const url = require('url');

const PORT = 3001;
const OPENAI_API_KEY = process.env.OPENAI_API_KEY || 'sk-demo-key';

const server = http.createServer(async (req, res) => {
  // CORS
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');

  if (req.method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }

  const parsedUrl = url.parse(req.url, true);

  if (parsedUrl.pathname === '/api/llm') {
    let body = '';
    req.on('data', chunk => body += chunk);
    req.on('end', async () => {
      try {
        const requestBody = JSON.parse(body);

        const options = {
          hostname: 'api.openai.com',
          port: 443,
          path: '/v1/chat/completions',
          method: 'POST',
          headers: {
            'Authorization': `Bearer ${OPENAI_API_KEY}`,
            'Content-Type': 'application/json',
          },
        };

        const proxyReq = https.request(options, (proxyRes) => {
          let data = '';
          proxyRes.on('data', chunk => data += chunk);
          proxyRes.on('end', () => {
            res.writeHead(proxyRes.statusCode, { 'Content-Type': 'application/json' });
            res.end(data);
          });
        });

        proxyReq.on('error', (err) => {
          res.writeHead(500, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: err.message }));
        });

        proxyReq.write(JSON.stringify({
          model: requestBody.model || 'gpt-4o-mini',
          messages: requestBody.messages,
          temperature: requestBody.temperature || 0.7,
          max_tokens: requestBody.max_tokens || 500,
        }));
        proxyReq.end();

      } catch (error) {
        res.writeHead(500, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: error.message }));
      }
    });
    return;
  }

  if (parsedUrl.pathname === '/api/mock') {
    let body = '';
    req.on('data', chunk => body += chunk);
    req.on('end', () => {
      const requestBody = JSON.parse(body);
      const task = requestBody.messages?.[0]?.content || 'desconocido';

      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({
        choices: [{
          message: {
            content: `Pensamiento: Analizando: "${task.substring(0, 50)}..."\nAccion: buscar\nEntrada de accion: ${task.substring(0, 30)}\nObservacion: resultados encontrados.`,
          },
          finish_reason: 'stop',
        }],
      }));
    });
    return;
  }

  res.writeHead(404);
  res.end('No encontrado');
});

server.listen(PORT, () => {
  console.log(`Proxy LLM de Loopscape ejecutandose en http://localhost:${PORT}`);
  console.log(`Rutas:`);
  console.log(`   POST /api/llm  - Proxy hacia OpenAI`);
  console.log(`   POST /api/mock - Respuestas simuladas sin clave API`);
});
