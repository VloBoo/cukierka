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

window.onload = async function() {
    const usersResponse = await sendSql("SELECT id, email FROM Users");
    const users = usersResponse.rows;

    const vacanciesResponse = await sendSql("SELECT id, title FROM Vacancies");
    const vacancies = vacanciesResponse.rows;

    const responsesResponse = await sendSql("SELECT * FROM Responses");
    const responses = responsesResponse.rows;

    const tableBody = document.getElementById("responsesTableBody");

    responses.forEach(response => {
        const row = document.createElement("tr");
        row.innerHTML = `
            <td>${response.id}</td>
            <td>
                <select id="user_${response.id}">
                    <option value="null">Пусто</option>
                    <!-- Users will be dynamically loaded here -->
                </select>
            </td>
            <td>
                <select id="vacancy_${response.id}">
                    <option value="null">Пусто</option>
                    <!-- Vacancies will be dynamically loaded here -->
                </select>
            </td>
            <td><input type="text" id="body_${response.id}" value="${response.body}"></td>
            <td>${response.created}</td>
            <td>
                <button onclick="updateResponse('${response.id}')">Update</button>
                <button onclick="deleteResponse('${response.id}')">Delete</button>
            </td>
        `;
        tableBody.appendChild(row);

        // Populate the dropdown with users
        const userSelect = document.getElementById(`user_${response.id}`);
        users.forEach(user => {
            const option = document.createElement("option");
            option.value = user.id;
            option.text = user.email;
            if (user.id === response.user_id) {
                option.selected = true;
            }
            userSelect.appendChild(option);
        });

        // Populate the dropdown with vacancies
        const vacancySelect = document.getElementById(`vacancy_${response.id}`);
        vacancies.forEach(vacancy => {
            const option = document.createElement("option");
            option.value = vacancy.id;
            option.text = vacancy.title;
            if (vacancy.id === response.vacancy_id) {
                option.selected = true;
            }
            vacancySelect.appendChild(option);
        });
    });
};

async function updateResponse(responseId) {
    const userId = document.getElementById(`user_${responseId}`).value;
    const vacancyId = document.getElementById(`vacancy_${responseId}`).value;
    const body = document.getElementById(`body_${responseId}`).value;

    if (userId == 'null' || vacancyId == 'null')  {
        alert("Выберите вторичный ключ");
        return
    }

    const sql = `UPDATE Responses SET user_id='${userId}', vacancy_id='${vacancyId}', body='${body}' WHERE id='${responseId}'`;
    await sendSql(sql);
    alert("Запрос на изменение отправлены на сервер!");
}

async function deleteResponse(responseId) {
    const confirmDelete = confirm("Вы действительно хотите удалить?");
    if (confirmDelete) {
        const sql = `DELETE FROM Responses WHERE id='${responseId}'`;
        await sendSql(sql);
        alert("Запрос на удаление отправлен на сервер!");
        // Reload the page after deletion
        location.reload();
    }
}
