CREATE TABLE Users(
    id UUID PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL, 
    other JSONB 
);  

CREATE TABLE Projects(
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    applications VARCHAR[] NOT NULL,
    other JSONB 
);  

CREATE TABLE UsersToProjects(
    user_id UUID REFERENCES Users(id),
    project_id UUID REFERENCES Projects(id),
    user_role VARCHAR NOT NULL
);  