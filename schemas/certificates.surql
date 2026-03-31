DEFINE TABLE IF NOT EXISTS certificates SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON certificates TYPE record<users>;
DEFINE FIELD IF NOT EXISTS course ON certificates TYPE record<courses>;
DEFINE FIELD IF NOT EXISTS certificate_number ON certificates TYPE string;
DEFINE FIELD IF NOT EXISTS issued_at ON certificates TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS pdf_url ON certificates TYPE option<string>;

DEFINE INDEX IF NOT EXISTS certificate_number_idx ON certificates FIELDS certificate_number UNIQUE;
DEFINE INDEX IF NOT EXISTS cert_user_course_idx ON certificates FIELDS user, course UNIQUE;
