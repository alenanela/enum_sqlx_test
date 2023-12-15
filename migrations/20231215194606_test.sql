CREATE TYPE measure_unit AS ENUM (
    'g'
);

CREATE TABLE nutrient (
    id int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    measure_unit measure_unit NOT NULL,
    name TEXT NOT NULL
);