-- Graph edge: user favorited a mosque
DEFINE TABLE IF NOT EXISTS favorited SCHEMAFULL TYPE RELATION IN users OUT mosques;

DEFINE FIELD IF NOT EXISTS created_at ON favorited TYPE datetime DEFAULT time::now();
