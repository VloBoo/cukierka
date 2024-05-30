CREATE TABLE Users(
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    information VARCHAR,
    created TIMESTAMPTZ NOT NULL
);

CREATE TABLE Tokens(
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users (id) ON DELETE CASCADE
);

CREATE TABLE Vacancies(
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL,
    title VARCHAR NOT NULL,
    information VARCHAR NOT NULL,
    payment INTEGER NOT NULL,
    status VARCHAR NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (author_id) REFERENCES Users (id) ON DELETE CASCADE
);

CREATE TABLE Responses(
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    vacancy_id UUID NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users (id) ON DELETE CASCADE,
    FOREIGN KEY (vacancy_id) REFERENCES Vacancies (id) ON DELETE CASCADE
);

CREATE TABLE Projects(
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    vacancy_id UUID NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users (id) ON DELETE CASCADE,
    FOREIGN KEY (vacancy_id) REFERENCES Vacancies (id) ON DELETE CASCADE
);

CREATE TABLE Messages(
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL,
    project_id UUID NOT NULL,
    content VARCHAR NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (author_id) REFERENCES Users (id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES Projects (id) ON DELETE CASCADE
);

CREATE TABLE Comments(
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL,
    user_id UUID NOT NULL,
    rate INT NOT NULL,
    content VARCHAR NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (author_id) REFERENCES Users (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES Users (id) ON DELETE CASCADE
);

CREATE VIEW MonthlyVacancies AS
SELECT
    DATE_TRUNC('month', created) AS month,
    COUNT(id) AS vacancy_count
FROM
    Vacancies
GROUP BY
    month
ORDER BY
    month;

CREATE VIEW VacancyResponses AS
SELECT
    v.id AS vacancy_id,
    v.title AS vacancy_title,
    COUNT(r.id) AS response_count
FROM
    Vacancies v
LEFT JOIN
    Responses r ON v.id = r.vacancy_id
GROUP BY
    v.id, v.title
ORDER BY
    response_count DESC;

CREATE VIEW VacancyStatusDistribution AS
SELECT
    status,
    COUNT(id) AS status_count
FROM
    Vacancies
GROUP BY
    status
ORDER BY
    status_count DESC;
