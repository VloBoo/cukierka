async function create() {
    const id = await getQueryParam('id');
    let message = document.getElementById('message__m').value;
    let rate = Number(document.getElementById('rate__m').value);
    if (rate == null || rate < 0 || rate > 10) {
        alert("Проверьте вводимые данные");
    }
    createComment(null, id, rate, message).then(function (response) {
        if (response.status === "ok") {
            window.location.reload();
        } else {
            alert("Не удалось оставить комментарий")
        }
    });
}

async function comment() {
    const id = await getQueryParam('id');
    const response = await sendSql("SELECT c.*, u.name AS commenter_name FROM Comments c JOIN Users u ON c.author_id = u.id WHERE c.user_id = '" + id + "';");

    // Получаем список сообщений из SQL-запроса
    const messages = response.rows;

    const tableBody = document.querySelector('.table-hover.table-striped tbody');

    // Очищаем содержимое таблицы перед добавлением новых данных
    tableBody.innerHTML = '';

    // Перебираем список сообщений и создаем элементы для каждого сообщения
    messages.forEach(function (message) {
        const userName = message.commenter_name;
        const messageText = message.content;
        const messageRate = message.rate;

        const title = userName + " (" + messageRate + "/10): " + messageText;
        const commentC = document.createElement('tr');
        const commentCell = document.createElement('td');
        const commentLink = document.createElement('a');
        commentLink.setAttribute('href', '/user?id=' + message.author_id); // Добавляем параметр ID к ссылке
        commentLink.textContent = title; // Установка текста ссылки на название вакансии
        commentCell.appendChild(commentLink);
        commentC.appendChild(commentCell);
        tableBody.appendChild(commentC);
    });
}