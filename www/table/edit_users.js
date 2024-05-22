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
    const usersResponse = await sendSql("SELECT * FROM Users");
    const resumesResponse = await sendSql("SELECT * FROM Resumes");

    const users = usersResponse.rows;
    const resumes = resumesResponse.rows;

    const tableBody = document.getElementById("usersTableBody");

    users.forEach(user => {
        const row = document.createElement("tr");
        row.innerHTML = `
            <td>${user.id}</td>
            <td><input type="text" value="${user.email}" id="email_${user.id}"></td>
            <td><input type="text" value="${user.firstname}" id="firstname_${user.id}"></td>
            <td><input type="text" value="${user.secondname}" id="secondname_${user.id}"></td>
            <td>
                <select id="resume_${user.id}">
                    <option value="null">Пусто</option>
                    <!-- Resumes will be dynamically loaded here -->
                </select>
            </td>
            <td>${user.created}</td>
            <td>
                <button onclick="updateUser('${user.id}')">Update</button>
                <button onclick="deleteUser('${user.id}')">Delete</button>
            </td>
        `;
        tableBody.appendChild(row);

        // Populate the dropdown with resumes
        const selectElement = document.getElementById(`resume_${user.id}`);
        resumes.forEach(resume => {
            const option = document.createElement("option");
            option.value = resume.id;
            option.text = resume.title;
            if (resume.id === user.resume_id) {
                option.selected = true;
            }
            selectElement.appendChild(option);
        });
    });
};

async function updateUser(userId) {
    const email = document.getElementById(`email_${userId}`).value;
    const firstname = document.getElementById(`firstname_${userId}`).value;
    const secondname = document.getElementById(`secondname_${userId}`).value;
    resumeId = document.getElementById(`resume_${userId}`).value;

    if (resumeId !== 'null') {
        resumeId = "'" + resumeId + "'";
    }

    const sql = `UPDATE Users SET email='${email}', firstname='${firstname}', secondname='${secondname}', resume_id=${resumeId} WHERE id='${userId}'`;
    await sendSql(sql);
    alert("Запрос на изменение отправлены на сервер!");
}

async function deleteUser(userId) {
    const confirmDelete = confirm("Вы действительно хотите удалить?");
    if (confirmDelete) {
        const sql = `DELETE FROM Users WHERE id='${userId}'`;
        await sendSql(sql);
        alert("Запрос на удаление отправлен на сервер!");
        // Reload the page after deletion
        location.reload();
    }
}
