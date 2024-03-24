CREATE VIEW CountVacanciesResponsesResumesByDay AS
SELECT
    date_series::date AS day,
    COUNT(DISTINCT v.id) AS vacancy_count,
    COUNT(DISTINCT rs.id) AS response_count,
    COUNT(DISTINCT u.id) AS resume_count
FROM generate_series(NOW() - INTERVAL '6 days', NOW(), INTERVAL '1 day') AS date_series
LEFT JOIN Vacancies v ON DATE_TRUNC('day', v.created) = date_series::date
LEFT JOIN Responses rs ON DATE_TRUNC('day', rs.created) = date_series::date
LEFT JOIN Users u ON v.user_id = u.id
LEFT JOIN Resumes r ON u.resume_id = r.id
GROUP BY day
ORDER BY day;

CREATE VIEW Top10PopularSkills AS
SELECT skill_item AS skill, COUNT(*) AS skill_count
FROM Vacancies, unnest(Vacancies.skill) AS skill_item
GROUP BY skill_item
ORDER BY skill_count DESC
LIMIT 5;

CREATE VIEW CountVacanciesResumes AS
SELECT
    (SELECT COUNT(*) FROM Vacancies) AS vacancy_count ,
    (SELECT COUNT(*) FROM Resumes) AS resume_count;
