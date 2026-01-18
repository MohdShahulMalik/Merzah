import { defineSchema, string, record, datetime, index, event } from 'smig';

export const sessions = defineSchema({
  table: 'sessions',
  fields: {
    user: record('users'),
    session_token: string(),
    created_at: datetime().default('time::now()'),
    expires_at: datetime(),
  },
  indexes: {
    idx_session_token: index(['session_token']).unique(),
  },
  events: {
    cleanup_expired_session: event('cleanup_expired_session')
      .onCreate()
      .onUpdate()
      .thenDo('DELETE sessions WHERE expires_at <= time::now()'),
  }
});
