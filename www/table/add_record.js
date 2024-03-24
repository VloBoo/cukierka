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

window.onload = async function () {
    // Load resumes for dropdowns
    const resumesResponse = await sendSql("SELECT id, title FROM Resumes");
    const resumes = resumesResponse.rows;

    const userSelects = document.querySelectorAll('#addUsersForm select[name="resume_id"]');
    userSelects.forEach(select => {
        resumes.forEach(resume => {
            const option = document.createElement('option');
            option.value = resume.id;
            option.textContent = resume.title;
            select.appendChild(option);
        });
    });

    // Load users for dropdowns
    const usersResponse = await sendSql("SELECT id, email FROM Users");
    const users = usersResponse.rows;

    const vacancyUserSelects = document.querySelectorAll('#addVacanciesForm select[name="user_id"]');
    vacancyUserSelects.forEach(select => {
        users.forEach(user => {
            const option = document.createElement('option');
            option.value = user.id;
            option.textContent = user.email;
            select.appendChild(option);
        });
    });

    const responseUserSelects = document.querySelectorAll('#addResponsesForm select[name="user_id"]');
    responseUserSelects.forEach(select => {
        users.forEach(user => {
            const option = document.createElement('option');
            option.value = user.id;
            option.textContent = user.email;
            select.appendChild(option);
        });
    });

    // Load vacancies for dropdowns
    const vacanciesResponse = await sendSql("SELECT id, title FROM Vacancies");
    const vacancies = vacanciesResponse.rows;

    const responseVacancySelects = document.querySelectorAll('#addResponsesForm select[name="vacancy_id"]');
    responseVacancySelects.forEach(select => {
        vacancies.forEach(vacancy => {
            const option = document.createElement('option');
            option.value = vacancy.id;
            option.textContent = vacancy.title;
            select.appendChild(option);
        });
    });
};

// Add record to Resumes
document.getElementById('addResumesForm').addEventListener('submit', async function (event) {
    event.preventDefault();

    const formData = new FormData(this);
    const data = {};
    formData.forEach((value, key) => {
        data[key] = value;
    });

    // Generate UUID for id
    data.id = await crypto.randomUUID();

    const sql = `INSERT INTO Resumes (id, title, body, payment, skill, created) VALUES ('${data.id}', '${data.title}', '${data.body}', '${data.payment}', ARRAY['${data.skill}']::VARCHAR[], NOW())`;
    await sendSql(sql);
    alert("Resume added successfully!");
});

// Add record to Users
document.getElementById('addUsersForm').addEventListener('submit', async function (event) {
    event.preventDefault();

    const formData = new FormData(this);
    const data = {};
    formData.forEach((value, key) => {
        data[key] = value;
    });

    // Generate UUID for id
    const userId = await crypto.randomUUID();

    const sql = `INSERT INTO Users (id, email, firstname, secondname, password, resume_id, type, created) VALUES ('${userId}', '${data.email}', '${data.firstname}', '${data.secondname}', '${data.password}', '${data.resume_id}', '${data.type}', NOW())`;
    await sendSql(sql);
    alert("User added successfully!");
});

// Add record to Vacancies
document.getElementById('addVacanciesForm').addEventListener('submit', async function (event) {
    event.preventDefault();

    const formData = new FormData(this);
    const data = {};
    formData.forEach((value, key) => {
        data[key] = value;
    });

    // Generate UUID for id
    const vacancyId = await crypto.randomUUID();

    const sql = `INSERT INTO Vacancies (id, user_id, title, body, payment, skill, created) VALUES ('${vacancyId}', '${data.user_id}', '${data.title}', '${data.body}', '${data.payment}', ARRAY['${data.skill}']::VARCHAR[], NOW())`;
    await sendSql(sql);
    alert("Vacancy added successfully!");
});

// Add record to Responses
document.getElementById('addResponsesForm').addEventListener('submit', async function (event) {
    event.preventDefault();

    const formData = new FormData(this);
    const data = {};
    formData.forEach((value, key) => {
        data[key] = value;
    });

    // Generate UUID for id
    const responseId = await crypto.randomUUID();

    const sql = `INSERT INTO Responses (id, user_id, vacancy_id, body, created) VALUES ('${responseId}', '${data.user_id}', '${data.vacancy_id}', '${data.body}', NOW())`;
    await sendSql(sql);
    alert("Response added successfully!");
});

