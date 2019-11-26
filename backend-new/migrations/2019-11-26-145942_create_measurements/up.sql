CREATE TABLE if not exists measurements (
    date TEXT PRIMARY KEY not null,
    temperature REAL not null,
    schleimstruktur TEXT not null,
    geschlechtsverkehr INTEGER not null,
    mittelschmerz INTEGER not null,
    zwischenblutung INTEGER not null,
    blutung TEXT not null
)