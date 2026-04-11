import type { PeerMessage } from '../types.ts';

type PeerMessageHandler = (message: PeerMessage) => void;

/**
 * Stateless relay for messages between personas.
 *
 * The router holds no history and no turn state — it is a routing table.
 * Personas subscribe when their Frontman starts; send() looks up the
 * handler for the target persona and invokes it synchronously.
 */
export class MessageRouter {
  private readonly handlers = new Map<string, PeerMessageHandler>();

  subscribe(personaId: string, handler: PeerMessageHandler): () => void {
    this.handlers.set(personaId, handler);
    return () => {
      if (this.handlers.get(personaId) === handler) {
        this.handlers.delete(personaId);
      }
    };
  }

  async send(
    params: Omit<PeerMessage, 'timestamp'> & { timestamp?: string },
  ): Promise<void> {
    const handler = this.handlers.get(params.to);
    if (!handler) {
      throw new Error(`Persona "${params.to}" is not subscribed to the message router`);
    }
    const message: PeerMessage = {
      from: params.from,
      to: params.to,
      message: params.message,
      replyTo: params.replyTo,
      timestamp: params.timestamp ?? new Date().toISOString(),
    };
    handler(message);
  }
}
