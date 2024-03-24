async function sendSql(sql) {
    const response = await fetch("/api/sql", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ sql: sql })
    });
    const data = await response.json();
    return data;
}


async function generateCSV() {
    const responseResumes = await sendSql("SELECT * FROM Resumes");
    const responseUsers = await sendSql("SELECT * FROM Users");
    const responseVacancies = await sendSql("SELECT * FROM Vacancies");
    const responseResponses = await sendSql("SELECT * FROM Responses");

    const resumes = responseResumes.rows;
    const users = responseUsers.rows;
    const vacancies = responseVacancies.rows;
    const responses = responseResponses.rows;

    const csvContent = "data:text/csv;charset=utf-8," +
        "Resumes:\n" +
        "ID,Title,Body,Payment,Skill,Created\n" +
        resumes.map(resume => `${resume.id},${resume.title},${resume.body},${resume.payment},"${resume.skill.join(', ')}",${resume.created}`).join('\n') +
        "\n\nUsers:\n" +
        "ID,Resume ID,Email,Firstname,Secondname,Password,Created,Type\n" +
        users.map(user => `${user.id},${user.resume_id},${user.email},${user.firstname},${user.secondname},${user.password},${user.created},${user.type}`).join('\n') +
        "\n\nVacancies:\n" +
        "ID,User ID,Title,Body,Payment,Skill,Created\n" +
        vacancies.map(vacancy => `${vacancy.id},${vacancy.user_id},${vacancy.title},${vacancy.body},${vacancy.payment},"${vacancy.skill.join(', ')}",${vacancy.created}`).join('\n') +
        "\n\nResponses:\n" +
        "ID,User ID,Vacancy ID,Body,Created\n" +
        responses.map(response => `${response.id},${response.user_id},${response.vacancy_id},${response.body},${response.created}`).join('\n');

    const encodedUri = encodeURI(csvContent);
    const link = document.createElement("a");
    link.setAttribute("href", encodedUri);
    link.setAttribute("download", "database_report.csv");
    document.body.appendChild(link);
    link.click();
}