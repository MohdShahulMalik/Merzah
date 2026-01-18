import { defineRelation, datetime, record, index } from 'smig';

export const handles = defineRelation({
  name: 'handles',
  from: 'users',
  to: 'mosques',
  fields: {
    granted_at: datetime().default('time::now()'),
    granted_by: record('users'),
  },
  indexes: {
    idx_handles_unique: index(['in', 'out']).unique(),
  }
});
