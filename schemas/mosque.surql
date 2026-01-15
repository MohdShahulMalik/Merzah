DEFINE TABLE IF NOT EXISTS mosques SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON mosques TYPE option<string>;
DEFINE FIELD IF NOT EXISTS location ON mosques TYPE geometry<point>;
DEFINE FIELD IF NOT EXISTS street ON mosques TYPE option<string>;
DEFINE FIELD IF NOT EXISTS city ON mosques TYPE option<string>;

DEFINE FIELD IF NOT EXISTS adhan_times ON mosques TYPE option<object>;
DEFINE FIELD IF NOT EXISTS adhan_times.fajr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.sunrise ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.dhuhr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.asr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.maghrib ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.isha ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.jummah ON mosques TYPE string;

DEFINE FIELD IF NOT EXISTS jamat_times ON mosques TYPE option<object>;
DEFINE FIELD IF NOT EXISTS jamat_times.fajr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.dhuhr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.asr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.maghrib ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.isha ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.jummah ON mosques TYPE string;

DEFINE FIELD IF NOT EXISTS created_at ON mosques TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON mosques TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS mosque_location_idx ON TABLE mosques COLUMNS location;
DEFINE INDEX IF NOT EXISTS idx_mosque_name ON TABLE mosques COLUMNS name;
DEFINE INDEX IF NOT EXISTS idx_mosque_city ON TABLE mosques COLUMNS city;
