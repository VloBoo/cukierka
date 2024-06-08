
async function create() {
    let title = document.getElementById('title').value;
    let information = document.getElementById('information').value;
    let status = document.getElementById('status').value;
    let payment = Number(document.getElementById('payment').value);

    createVacancy(null, title, information, payment, status).then(function (response) {
        if (response.status === "ok") {
            alert("Вакансия успешно создана!")
            window.location.href = "/vacancy?id=" + response.vacancy;
        } else {
            alert("Не удалось создать вакансию")
        }
    });
}

async function vacancy() {
    getQueryParam('id').then(function (id) {
        getVacancyById(id).then(function (response) {
            let token = getCookie("token");
            document.getElementById('title').value = response.vacancy.title;
            document.getElementById('information').value = response.vacancy.information;
            document.getElementById('status').value = response.vacancy.status;
            document.getElementById('payment').value = response.vacancy.payment;
            document.getElementById('response_link').href = '/response?id=' + response.vacancy.id
            // console.log("SELECT id FROM Vacancies WHERE id = '" + id + "' AND (author_id = (SELECT user_id FROM Tokens WHERE id = '" + token + "') OR 'cf05dc50-8966-4418-9d99-ce0f293f525d' = (SELECT user_id FROM Tokens WHERE id = '" + token + "'));")
            sendSql("SELECT id FROM Vacancies WHERE id = '" + id + "' AND (author_id = (SELECT user_id FROM Tokens WHERE id = '" + token + "') OR 'cf05dc50-8966-4418-9d99-ce0f293f525d' = (SELECT user_id FROM Tokens WHERE id = '" + token + "'));").then(function (response2) {
                if (response2.rows[0] !== null && response2.rows.length !== 0) {
                    document.getElementById('edit_link').href = '/vacancy/edit?id=' + response.vacancy.id
                    console.log(response2);
                } else {
                    document.getElementById('edit_link').remove();
                }
            });
            sendSql("SELECT id FROM Projects WHERE vacancy_id = '" + id + "';").then(function (response2) {
                if (response2.rows[0].id !== null && response2.rows.length !== 0) {
                    document.getElementById('chat').href = '/chat?id=' + response2.rows[0].id;
                    console.log(response2);
                } else {
                    document.getElementById('chat').remove();
                }
            });
        });
    })
}

async function create_res() {
    getQueryParam('id').then(function (id) {
        createResponse(null, id).then(function (response) {
            if (response.status === "ok") {
                alert('Отклик оставлен!');
            } else {
                alert('Не удалось оставить отклик!');
            }
        });
    })
}

async function response() {
    const id = await getQueryParam('id');
    const response = await sendSql("SELECT u.id AS user_id, u.name AS name, r.id AS r_id FROM Users u JOIN Responses r ON u.id = r.user_id WHERE r.vacancy_id = '" + id + "';");

    // Получаем список откликов из SQL-запроса
    const rows = response.rows;

    // Получаем элемент таблицы, куда будем добавлять данные
    const tableBody = document.querySelector('.tm-table-striped-even tbody');

    // Очищаем содержимое таблицы перед добавлением новых данных
    tableBody.innerHTML = '';

    // Перебираем список откликов и создаем строки таблицы с именами пользователей
    rows.forEach(function (row, index) {
        const userId = row.user_id;
        const userName = row.name; // Нужно заменить на реальное имя пользователя из базы данных

        // Создаем новую строку таблицы
        const newRow = document.createElement('tr');

        // Создаем ячейку с кнопкой
        const buttonCell = document.createElement('td');
        const selectButton = document.createElement('button');
        selectButton.setAttribute('type', 'button');
        selectButton.setAttribute('class', 'btn btn-primary d-inline-block mx-auto');
        selectButton.textContent = 'Выбрать пользователя для вакансии';

        // Добавляем обработчик события при нажатии на кнопку
        selectButton.addEventListener('click', async function () {
            await createRroject_a(row.r_id); // Вызываем функцию createProject при нажатии на кнопку
        });

        // Добавляем кнопку в ячейку и ячейку в строку
        buttonCell.appendChild(selectButton);
        newRow.appendChild(buttonCell);

        // Создаем ячейку с ссылкой на профиль пользователя
        const userCell = document.createElement('td');
        const userLink = document.createElement('a');
        userLink.setAttribute('href', '/user?id=' + userId); // Добавляем параметр ID к ссылке
        userLink.textContent = userName; // Установка текста ссылки на имя пользователя
        userCell.appendChild(userLink);
        newRow.appendChild(userCell);

        // Добавляем новую строку в таблицу
        tableBody.appendChild(newRow);
    });
}

async function createRroject_a(id) {
    getQueryParam('id').then(function (id2) {
        createProject(null, id, id2).then(function (response) {
            if (response.status === "ok") {
                alert("Пользователь успешно выбран!");
                window.location.href = "/vacancy?id=" + id2;
            } else {
                alert("Не удалось выбрать пользователя");
            }
        })
    })
}

async function search() {
    let name = document.getElementById('name_s').value;
    let s = document.getElementById('sort_s').value;
    let s2 = document.getElementById('sort_s2').value;
    searchVacancies(name, s, s2).then(function (res) {
        console.log(res.vacancy);

        // Получаем элемент таблицы, куда будем добавлять вакансии
        const tableBody = document.querySelector('.table-hover.table-striped tbody');

        // Очищаем содержимое таблицы перед добавлением новых данных
        tableBody.innerHTML = '';

        (async function go(i) {
            if (i < res.vacancy.length) {
                const vacancy = await getVacancyById(res.vacancy[i]);
                const title = vacancy.vacancy.title + " (" + vacancy.vacancy.status + ")";
                // Создаем новую строку таблицы
                const newRow = document.createElement('tr');

                // Создаем ячейку с ссылкой на вакансию
                const vacancyCell = document.createElement('td');
                const vacancyLink = document.createElement('a');
                vacancyLink.setAttribute('href', '/vacancy?id=' + vacancy.vacancy.id); // Добавляем параметр ID к ссылке
                vacancyLink.textContent = title; // Установка текста ссылки на название вакансии
                vacancyCell.appendChild(vacancyLink);
                newRow.appendChild(vacancyCell);

                // Добавляем новую строку в таблицу
                tableBody.appendChild(newRow);

                go(i + 1);
                console.log(i);

            }
        })(0);

       
    })
}

async function my() {
    let token = getCookie("token");
    if (token === null) {
        window.location.href = "/login";
        return;
    }

    // Пример SQL-запроса для получения списка вакансий и их идентификаторов
    const response = await sendSql("SELECT v.id FROM Vacancies v WHERE v.author_id = (SELECT user_id FROM Tokens WHERE id = '" + token + "') UNION SELECT v.id FROM Vacancies v JOIN Responses r ON v.id = r.vacancy_id WHERE r.user_id = (SELECT user_id FROM Tokens WHERE id = '" + token + "');");

    // Получаем список вакансий из SQL-запроса
    const vacancies = response.rows;

    // Получаем элемент таблицы, куда будем добавлять вакансии
    const tableBody = document.querySelector('.table-hover.table-striped tbody');

    // Очищаем содержимое таблицы перед добавлением новых данных
    tableBody.innerHTML = '';

    // Перебираем список вакансий и создаем строки таблицы с названиями вакансий и ссылками
    vacancies.forEach(async function (v_id) {
        const vacancy = await getVacancyById(v_id.id);
        const title = vacancy.vacancy.title + " (" + vacancy.vacancy.status + ")";
        // Создаем новую строку таблицы
        const newRow = document.createElement('tr');

        // Создаем ячейку с ссылкой на вакансию
        const vacancyCell = document.createElement('td');
        const vacancyLink = document.createElement('a');
        vacancyLink.setAttribute('href', '/vacancy?id=' + vacancy.vacancy.id); // Добавляем параметр ID к ссылке
        vacancyLink.textContent = title; // Установка текста ссылки на название вакансии
        vacancyCell.appendChild(vacancyLink);
        newRow.appendChild(vacancyCell);

        // Добавляем новую строку в таблицу
        tableBody.appendChild(newRow);
    });
}


async function vacancy_e() {
    getQueryParam('id').then(function (id) {
        getVacancyById(id).then(function (response) {
            let token = getCookie("token");
            document.getElementById('title').value = response.vacancy.title;
            document.getElementById('information').value = response.vacancy.information;
            document.getElementById('status').value = response.vacancy.status;
            document.getElementById('payment').value = response.vacancy.payment;
            document.getElementById('response_link').href = '/response?id=' + response.vacancy.id
        });
    })
}


async function create_e() {
    getQueryParam('id').then(function (id) {
        let title = document.getElementById('title').value;
        let information = document.getElementById('information').value;
        let status = document.getElementById('status').value;
        let payment = Number(document.getElementById('payment').value);

        updateVacancy(id, null, title, information, payment, status).then(function (response) {
            if (response.status === "ok") {
                alert("Вакансия успешно обновлена!")
                window.location.href = "/vacancy?id=" + response.vacancy;
            } else {
                alert("Не удалось обновить вакансию")
            }
        });
    })
}
