async function message() {
    const id = await getQueryParam('id');
    const response = await sendSql("SELECT u.name AS user_name, m.content AS message_text, m.created AS created FROM Messages m JOIN Users u ON m.author_id = u.id WHERE m.project_id = '" + id + "' ORDER BY m.created ASC;");

    // Получаем список сообщений из SQL-запроса
    const messages = response.rows;

    // Получаем форму чата, куда будем добавлять сообщения
    // Получаем форму чата, куда будем добавлять сообщения
    const chatForm = document.getElementById('chat');

    // Очищаем содержимое формы чата перед добавлением новых сообщений
    chatForm.innerHTML = '';

    // Перебираем список сообщений и создаем элементы для каждого сообщения
    messages.forEach(function (message, index) {
        const userName = message.user_name;
        const messageText = message.message_text;

        // Создаем новый блок сообщения
        const messageBlock = document.createElement('div');
        messageBlock.classList.add('form-group');

        // Добавляем имя пользователя
        const nameLabel = document.createElement('label');
        nameLabel.textContent = userName;
        messageBlock.appendChild(nameLabel);

        // Добавляем текст сообщения
        const messageInput = document.createElement('input');
        messageInput.value = messageText;
        messageInput.classList.add('form-control', 'validate');
        messageInput.setAttribute('disabled', 'disabled');
        messageBlock.appendChild(messageInput);

        // Добавляем блок сообщения в форму чата
        chatForm.appendChild(messageBlock);
    });
}


async function create() {
    const id = await getQueryParam('id');
    let message = document.getElementById('message__m').value ;
    createMessage(null, id, message).then(function (response) {
        if (response.status === "ok") {
            window.location.reload();
        } else {
            alert("Не удалось отправить сообщение")
        }
    });
}
