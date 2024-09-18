export const MESSAGE = {
    proof_request: 'proof_request',
    proof_done: 'proof_done',
    proof_error: 'proof_error',
} as const;

export type MessageType = typeof MESSAGE[keyof typeof MESSAGE];