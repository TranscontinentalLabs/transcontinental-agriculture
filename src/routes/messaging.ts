import { Router, Request, Response } from 'express';

export const messagingRouter = Router();

messagingRouter.post('/user-message', async (req: Request, res: Response) => {
  const { user_message, session_id = 'session-' + Date.now() } = req.body || {};
  const n8nUrl = process.env.N8N_WEBHOOK_URL || 'http://localhost:5678/webhook/interpret-user-message';

  try {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 2000);
    const webhookRes = await fetch(n8nUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: user_message, session_id }),
      signal: controller.signal,
    });
    clearTimeout(timeout);

    if (webhookRes.ok) {
      const data = await webhookRes.json();
      return res.json({ session_id, message: data.text || data.message || 'Processed by workflow' });
    }
  } catch {
    // Graceful in-memory fallback when n8n is offline or not configured
  }

  // Sourced intent & entity extraction fallback
  const text = String(user_message || '').toLowerCase();
  let intent = 'propose_new_greenhouse';
  const crops: string[] = [];
  if (text.includes('tomato')) crops.push('tomato');
  if (text.includes('capsicum') || text.includes('pepper')) crops.push('capsicum');
  if (crops.length === 0) crops.push('tomato');

  return res.json({
    session_id,
    message: `Interpreted request for ${crops.join(', ')}. Candidate greenhouse: Venlo 1,036.8 m² with thermal/shading screens and closed-loop fertigation.`,
    structured_intent: {
      intent,
      crops,
      user_message,
      status: 'ready_for_recommendation',
    },
  });
});
