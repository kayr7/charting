
table! {
    use diesel::sql_types::*;
    measurements (date) {
        date -> Text,
        Temperature -> Double,
        Schleimstruktur -> Text,
        Geschlechtsverkehr -> Integer,
        Mittelschmerz -> Integer,
        Zwischenblutung -> Integer,
        Blutung -> Text,
    }
}
