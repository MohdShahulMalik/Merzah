import { defineSchema, string, datetime } from 'smig';

export const users = defineSchema({
  table: 'users',
  fields: {
    password_hash: string().assert('string::len($value) > 0'),
    role: string()
      .assert("$value IN ['app_admin', 'educator', 'regular']")
      .default("'regular'"),
    display_name: string(),
    created_at: datetime().default('time::now()'),
    updated_at: datetime().default('time::now()'),
  }
});