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

    const vacanciesResponse = await sendSql("SELECT * FROM Vacancies");
    const vacancies = vacanciesResponse.rows;

    const tableBody = document.getElementById("vacanciesTableBody");

    vacancies.forEach(vacancy => {
        const row = document.createElement("tr");
        row.innerHTML = `
            <td>${vacancy.id}</td>
            <td>
                <select id="user_${vacancy.id}">
                    <option value="null">Пусто</option>
                    <!-- Users will be dynamically loaded here -->
                </select>
            </td>
            <td><input type="text" id="title_${vacancy.id}" value="${vacancy.title}"></td>
            <td><input type="text" id="body_${vacancy.id}" value="${vacancy.body}"></td>
            <td><input type="number" id="payment_${vacancy.id}" value="${vacancy.payment}"></td>
            <td><input type="text" id="skill_${vacancy.id}" value="${vacancy.skill.join(', ')}"></td>
            <td>${vacancy.created}</td>
            <td>
                <button onclick="updateVacancy('${vacancy.id}')">Update</button>
                <button onclick="deleteVacancy('${vacancy.id}')">Delete</button>
            </td>
        `;
        tableBody.appendChild(row);

        // Populate the dropdown with users
        const selectElement = document.getElementById(`user_${vacancy.id}`);
        users.forEach(user => {
            const option = document.createElement("option");
            option.value = user.id;
            option.text = user.email;
            if (user.id === vacancy.user_id) {
                option.selected = true;
            }
            selectElement.appendChild(option);
        });
    });
};

async function updateVacancy(vacancyId) {
    const userId = document.getElementById(`user_${vacancyId}`).value;
    const title = document.getElementById(`title_${vacancyId}`).value;
    const body = document.getElementById(`body_${vacancyId}`).value;
    const payment = document.getElementById(`payment_${vacancyId}`).value;
    const skill = document.getElementById(`skill_${vacancyId}`).value.split(', ');

    if (userId == 'null') {
        alert("Выберите вторичный ключ");
        return
    }

    const sql = `UPDATE Vacancies SET user_id='${userId}', title='${title}', body='${body}', payment=${payment}, skill='{${skill}}' WHERE id='${vacancyId}'`;
    await sendSql(sql);
    alert("Запрос на изменение отправлены на сервер!");
}

async function deleteVacancy(vacancyId) {
    const confirmDelete = confirm("Вы действительно хотите удалить?");
    if (confirmDelete) {
        const sql = `DELETE FROM Vacancies WHERE id='${vacancyId}'`;
        await sendSql(sql);
        alert("Запрос на удаление отправлен на сервер!");
        // Reload the page after deletion
        location.reload();
    }
}
