import {
  // Field types
  string,
  int,
  float,
  bool,
  datetime,
  uuid,
  array,
  record,
  // Indexes
  index,
  // Schema builders
  defineSchema,
  defineRelation,
  composeSchema,
  // Common patterns
  cf,
  ci,
  ce,
  // Entities (SurrealDB 3.x)
  fn,
  analyzer,
  access,
  param,
} from 'smig';

/**
 * smig Schema File (SurrealDB 3.x)
 *
 * This template showcases smig’s full capabilities:
 * - Type-safe schema definitions
 * - Vector indexes for AI/ML (HNSW, MTREE)
 * - ACCESS authentication (JWT, RECORD)
 * - Rename tracking with .was()
 * - And much more!
 */

// =============================================================================
// USER MODEL
// =============================================================================
const userModel = defineSchema({
  table: 'user',
  fields: {
    // Basic fields
    name: string().required(),
    email: string().required(),

    // Optional fields with defaults
    isActive: bool().default(true),
    role: string().default('user'),

    // Timestamps (using common field patterns)
    createdAt: cf.timestamp(),
    updatedAt: cf.timestamp(),

    // Vector embedding for AI search (384 dimensions)
    embedding: array('float').comment('User profile embedding'),
  },
  indexes: {
    // Unique email constraint
    emailIndex: index(['email']).unique(),

    // Vector index for similarity search (HNSW algorithm)
    embeddingIndex: index(['embedding'])
      .hnsw()
      .dimension(384)
      .dist('COSINE')
      .comment('AI-powered user search'),
  },
});

// =============================================================================
// POST MODEL
// =============================================================================
const postModel = defineSchema({
  table: 'post',
  fields: {
    title: string().required(),
    content: string(),
    author: record('user').required(),
    isPublished: bool().default(false),
    viewCount: int().default(0).readonly(),
    createdAt: cf.timestamp(),

    // Content embedding for semantic search
    contentEmbedding: array('float'),
  },
  indexes: {
    authorIndex: index(['author']),
    publishedIndex: index(['isPublished', 'createdAt']),

    // Full-text search with custom analyzer
    contentSearch: index(['content'])
      .search()
      .analyzer('english_analyzer')
      .highlights(),
  },
});

// =============================================================================
// LIKE RELATION (Graph Edge)
// =============================================================================
const likeRelation = defineRelation({
  name: 'like',
  from: 'user',
  to: 'post',
  fields: {
    createdAt: cf.timestamp(),
  },
});

// =============================================================================
// CUSTOM FUNCTION
// =============================================================================
const daysSinceFunction = fn('fn::days_since')
  .param('date', 'datetime')
  .returns('int')
  .body(`
    RETURN math::floor((time::now() - $date) / 86400);
  `);

// =============================================================================
// TEXT ANALYZER (Full-text Search)
// =============================================================================
const englishAnalyzer = analyzer('english_analyzer')
  .tokenizers(['blank', 'class'])
  .filters(['lowercase', 'snowball(english)']);

// =============================================================================
// ACCESS DEFINITION (Authentication)
// =============================================================================
const userAccess = access('user')
  .record()
  .signup(`
    CREATE user SET
      email = $email,
      password = crypto::argon2::generate($password)
  `)
  .signin(`
    SELECT * FROM user WHERE
      email = $email AND
      crypto::argon2::compare(password, $password)
  `)
  .session('7d');

// =============================================================================
// GLOBAL PARAMETER
// =============================================================================
const appVersion = param('app_version').value("'1.0.0'");

// =============================================================================
// COMPOSE COMPLETE SCHEMA
// =============================================================================
const fullSchema = composeSchema({
  models: {
    user: userModel,
    post: postModel,
  },
  relations: {
    like: likeRelation,
  },
  functions: {
    days_since: daysSinceFunction,
  },
  analyzers: {
    english_analyzer: englishAnalyzer,
  },
  scopes: {
    user: userAccess,
  },
});

export default fullSchema;
