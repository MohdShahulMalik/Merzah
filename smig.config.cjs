module.exports = {
  url: `ws://${Bun.env.SURREAL_URL }`,
  namespace: Bun.env.SURREAL_NS,
  database: Bun.env.SURREAL_NS,
  username: Bun.env.SURREAL_USER,
  password: Bun.env.SURREAL_PASS,
  schema: './db/schemas/index.ts',

  environments: {
    production: {
      namespace: Bun.env.SURREAL_PROD_NS,
      database: Bun.env.SURREAL_PROD_DB,
    },
  }
};
