import { defineSchema, string, record, datetime, index } from 'smig';

export const user_identifier = defineSchema({
  table: 'user_identifier',
  fields: {
    user: record('users'),
    identifier_type: string().assert("$value IN ['email', 'mobile']"),
    identifier_value: string(),
    created_at: datetime().default('time::now()'),
    updated_at: datetime().default('time::now()'),
  },
  indexes: {
    idx_identifier_value: index(['identifier_value']).unique(),
    idx_user_identifier_type: index(['user', 'identifier_type']).unique(),
  }
});