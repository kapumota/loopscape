// Cloudflare Worker: proxy LLM para Loopscape
// Despliegue: wrangler deploy

export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);

    // Cabeceras CORS
    const corsHeaders = {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type, Authorization',
    };

    if (request.method === 'OPTIONS') {
      return new Response(null, { headers: corsHeaders });
    }

    // Chequeo de salud
    if (url.pathname === '/health') {
      return new Response(JSON.stringify({ status: 'ok', service: 'loopscape-llm-proxy' }), {
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
      });
    }

    // Ruta de proxy LLM
    if (url.pathname === '/api/llm') {
      try {
        const body = await request.json();

        // Reenvia hacia OpenAI
        const response = await fetch('https://api.openai.com/v1/chat/completions', {
          method: 'POST',
          headers: {
            'Authorization': `Bearer ${env.OPENAI_API_KEY}`,
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            model: body.model || 'gpt-4o-mini',
            messages: body.messages,
            temperature: body.temperature || 0.7,
            max_tokens: body.max_tokens || 500,
          }),
        });

        const data = await response.json();

        return new Response(JSON.stringify(data), {
          status: response.status,
          headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        });

      } catch (error) {
        return new Response(JSON.stringify({ error: error.message }), {
          status: 500,
          headers: { ...corsHeaders, 'Content-Type': 'application/json' },
        });
      }
    }

    // Ruta simulada para pruebas sin clave API
    if (url.pathname === '/api/mock') {
      const body = await request.json();
      const task = body.messages?.[0]?.content || 'tarea desconocida';

      return new Response(JSON.stringify({
        choices: [{
          message: {
            content: `Pensamiento: Analizando la tarea: "${task.substring(0, 50)}..."\nAccion: buscar\nEntrada de accion: ${task.substring(0, 30)}\nObservacion: informacion relevante encontrada.`,
          },
          finish_reason: 'stop',
        }],
      }), {
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
      });
    }

    return new Response('No encontrado', { status: 404, headers: corsHeaders });
  },
};
