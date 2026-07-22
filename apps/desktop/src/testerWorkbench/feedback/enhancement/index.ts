/**
 * Governed SD-11 GitHub enhancement-request flow.
 *
 * Enhancement-only composition (`composeEnhancementRequest`) over the shared
 * feedback evidence substrate, plus governed submission
 * (`submitEnhancementRequest`) that never claims success without a real issue
 * handle and always preserves a copyable draft. The flow stays separate from the
 * bug-report flow while reusing the shared evidence schema so the two cannot
 * drift apart.
 */

export * from './composeEnhancementRequest';
export * from './submitEnhancementRequest';
