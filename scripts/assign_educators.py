#!/usr/bin/env python3
"""Reassign course educators by slug pattern (files + DB).

Overwrites educator_id in all scripts/data/courses/*.json based on slug
keywords, then applies the same values to the matching course records.
Surgical: only the educator_id line is touched, rest of file byte-identical.

Also creates dedicated educator user accounts for courses with no keyword
match (NEW_EDUCATORS below) and assigns those courses to them. Created
accounts get role "educator" and an unusable password hash plus no login
identifier, so they can never sign in - they exist for content attribution.
"""

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from import_db import REPO_DIR, esc, sql  # noqa: E402

DATA_DIR = os.path.join(REPO_DIR, "scripts", "data")
IDS_FILE = os.path.join(DATA_DIR, "educator_ids.json")
COURSES_DIR = os.path.join(DATA_DIR, "courses")

with open(IDS_FILE, encoding="utf-8") as fh:
    educators = json.load(fh)

edu_map = {e["name"]: e["id"] for e in educators}
id_to_name = {e["id"]: e["name"] for e in educators}

ASSIGNMENTS = {
    # Salafi/Athari sources verified against the seed files' actual video
    # authors (see scripts/data/educator_ids.json). Multi-word keys win over
    # shorter overlapping keys because Python dicts keep insertion order.
    "aqeedah-series": "Newcastle Central Mosque",
    "thalathat": "AMAU Academy",
    "qawaid": "AMAU Academy",
    "nawaid": "AMAU Academy",
    "forty-hadith": "Madeenah Academy",
    "hajj-in-perspective": "Garden of Ilm",
    "hajj-and-umrah": "Garden of Ilm",
    "hajj": "Garden of Ilm",
    "tafsir": "Garden of Ilm",
    "rightly-guided": "The Patient One",
    "caliph": "The Patient One",
    "seerah-in-years": "Muwahhideen Publications",
    "lessons-from-seerah": "Salafi Matloob",
    "life-of-prophet": "Forgotten Light",
    "seerah": "Muwahhideen Publications",
    "prophet-muhammad": "Forgotten Light",
    "the-salafi-woman": "AMAU Academy",
    "married-ever-after": "AMAU Academy",
    "secrets-to-successful-marriage": "AMAU Academy",
    "salafi-dawah": "Salafi Publications",
    "dawah": "Salafi Publications",
    "adab-doing-things": "Salafi Publications",
    "al-adab-al-mufrad": "Salafi Publications",
    "islamic-manners": "Salafi Publications",
    "manners": "Salafi Publications",
    "adab": "Salafi Publications",
    "patience": "Forgotten Light",
    "time-management": "AMAU Academy",
    "arabic": "Green Lane Masjid",
    "salah": "Newcastle Central Mosque",
    "fiqh": "Newcastle Central Mosque",
    "muslim-business-leadership": "Newcastle Central Mosque",
    "leadership-islamic-values": "Miftaah Institute",
    "starting-halal": "Assim Al-Hakeem",
    "halal-entrepreneurship": "Assim Al-Hakeem",
    "islamic-business-ethics": "Assim Al-Hakeem",
    "ethical-business-practices": "Assim Al-Hakeem",
    "riba": "Assim Al-Hakeem",
    "islamic-apps-tools": "Quran.com",
    "quran-recitation-basics": "E-Tilawah Academy",
    "learn-reading-quran": "Islamic Home Schooling Academy - New Zealand",
    "tajweed": "TROID",
    "recitation": "E-Tilawah Academy",
    "reading-quran": "Islamic Home Schooling Academy - New Zealand",
    "aqeedah": "Garden of Ilm",
    "tawheed": "Garden of Ilm",
    "quran": "Quran.com",
    "bukhari": "Imam al-Bukhari",
    "muslim": "Imam Muslim",
    "nawawi": "Imam an-Nawawi",
    "hadith": "Imam al-Bukhari",
    "zakat": "Imam Malik",
    "prophet": "Muwahhideen Publications",
    "character": "Salafi Publications",
    "married": "AMAU Academy",
    "marriage": "AMAU Academy",
    "family": "AMAU Academy",
    "finance": "SeekersGuidance",
    "investing": "SeekersGuidance",
    "inheritance": "SeekersGuidance",
    "business": "SeekersGuidance",
    "entrepreneurship": "SeekersGuidance",
    "leadership": "SeekersGuidance",
    "communication": "SeekersGuidance",
    "workplace": "SeekersGuidance",
    "ethics": "Shaykh Abdul-Rahim Reasat",
    "community": "Salafi Publications",
    "digital": "SeekersGuidance",
    "online-safety": "SeekersGuidance",
    "islamic-apps": "SeekersGuidance",
}

DEFAULT_EDUCATOR = educators[0]["name"]

# Slugs managed explicitly below (none - Salafi courses are attributed via
# ASSIGNMENTS to deterministic-id accounts seeded by
# populate_salafi_educators.sh, e.g. users:amau_academy, and are also
# re-synced to the DB here on every run).
MANAGED_SLUGS = set()

changed = []
for fname in sorted(os.listdir(COURSES_DIR)):
    if not fname.endswith(".json"):
        continue
    fpath = os.path.join(COURSES_DIR, fname)
    with open(fpath, encoding="utf-8") as fh:
        raw = fh.read()
    course = json.loads(raw)
    slug = course.get("slug", fname.replace(".json", ""))
    if slug in MANAGED_SLUGS:
        continue
    old_id = course.get("educator_id")

    assigned = None
    for pattern, name in ASSIGNMENTS.items():
        if pattern in slug:
            assigned = name
            break
    if assigned is None:
        assigned = DEFAULT_EDUCATOR
        print("  ! %s matched nothing, defaulting to %s" % (slug, assigned))
    new_id = edu_map[assigned]
    if old_id == new_id:
        continue

    old_fragment = '"educator_id": "%s"' % old_id
    assert raw.count(old_fragment) == 1, "unexpected educator line in %s" % fname
    with open(fpath, "w", encoding="utf-8") as fh:
        fh.write(raw.replace(old_fragment, '"educator_id": "%s"' % new_id, 1))
    changed.append((slug, old_id, new_id))
    print(
        "  %s: %s -> %s"
        % (slug, id_to_name.get(old_id, old_id), assigned)
    )

print("\nfiles updated: %d" % len(changed))

db_updated = 0
for slug, _old_id, new_id in changed:
    rows = sql(
        "SELECT VALUE id FROM courses WHERE slug = \"%s\" LIMIT 1"
        % slug.replace('"', '\\"')
    )
    if not rows:
        print("  ! course %s not in DB, skipped" % slug)
        continue
    course_id = rows[0] if isinstance(rows[0], str) else rows[0]["id"]
    sql(
        "UPDATE ONLY %s SET educator = %s, updated_at = time::now()"
        % (course_id, new_id)
    )
    db_updated += 1

print("db records updated: %d" % db_updated)

# --- Dedicated educators (retired) ----------------------------------------
# Previously created random-id users for mutoon courses. Those courses now
# point at deterministic-id Salafi accounts (users:amau_academy etc.) and are
# routed via ASSIGNMENTS above; keep the dict empty rather than delete the
# mechanism, in case a future dedicated account is needed.
NEW_EDUCATORS = {}

UNUSABLE_PASSWORD_HASH = "UNUSABLE_SEED_ACCOUNT"


def ensure_educator(display_name):
    rows = sql(
        "SELECT VALUE id FROM users WHERE display_name = %s LIMIT 1"
        % esc(display_name)
    )
    if rows:
        educator_id = rows[0] if isinstance(rows[0], str) else rows[0]["id"]
        print("  = educator %s already exists (%s)" % (display_name, educator_id))
        return educator_id
    created = sql(
        "CREATE users CONTENT { display_name: %s,"
        " password_hash: %s, role: %s,"
        " created_at: time::now(), updated_at: time::now() }"
        % (esc(display_name), esc(UNUSABLE_PASSWORD_HASH), esc("educator"))
    )
    educator_id = created[0]["id"] if isinstance(created[0], dict) else created[0]
    print("  + educator %s created (%s)" % (display_name, educator_id))
    return educator_id


def set_course_educator(slug, educator_id, educator_name):
    fpath = os.path.join(COURSES_DIR, slug + ".json")
    with open(fpath, encoding="utf-8") as fh:
        raw = fh.read()
    course = json.loads(raw)
    old_id = course.get("educator_id")
    if old_id != educator_id:
        old_fragment = '"educator_id": "%s"' % old_id
        assert raw.count(old_fragment) == 1, "unexpected educator line in %s" % fpath
        with open(fpath, "w", encoding="utf-8") as fh:
            fh.write(raw.replace(old_fragment, '"educator_id": "%s"' % educator_id, 1))
        print("  file %s: %s -> %s" % (slug, old_id, educator_name))
    rows = sql("SELECT VALUE id FROM courses WHERE slug = %s LIMIT 1" % esc(slug))
    if not rows:
        print("  ! course %s not in DB, skipped" % slug)
        return
    course_id = rows[0] if isinstance(rows[0], str) else rows[0]["id"]
    sql(
        "UPDATE ONLY %s SET educator = %s, updated_at = time::now()"
        % (course_id, educator_id)
    )
    print("  db   %s educator -> %s" % (slug, educator_name))


for display_name, slugs in NEW_EDUCATORS.items():
    educator_id = ensure_educator(display_name)
    for slug in slugs:
        set_course_educator(slug, educator_id, display_name)

# --- Seed educator accounts for educator_ids.json ---------------------------
# Creates real user records with the EXACT ids referenced by the course seed
# files, so educator links resolve instead of dangling. Attribution-only:
# role "educator", unusable password hash, no login identifiers.
# Only id/display_name are ever read back - password hashes are never
# selected, printed, or compared.
def ensure_seed_educator(display_name, rid):
    rows = sql("SELECT id, display_name FROM users WHERE id = %s LIMIT 1" % rid)
    if rows:
        if rows[0].get("display_name") != display_name:
            print(
                "  ! %s already exists with a different name (%s), skipped"
                % (rid, rows[0].get("display_name"))
            )
            return
        print("  = educator %s already exists" % display_name)
        return
    sql(
        "CREATE %s CONTENT { display_name: %s, password_hash: %s, role: %s,"
        " created_at: time::now(), updated_at: time::now() }"
        % (rid, esc(display_name), esc(UNUSABLE_PASSWORD_HASH), esc("educator"))
    )
    print("  + educator %s created (%s)" % (display_name, rid))


for _name, _rid in edu_map.items():
    ensure_seed_educator(_name, _rid)
