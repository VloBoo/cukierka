CREATE TABLE Users(
    id UUID PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL, 
    other JSONB 
);  