function getCookie(name) {
    var cookies = document.cookie.split(";"); // Разделяем строку куки на отдельные куки
    for (var i = 0; i < cookies.length; i++) {
        var cookie = cookies[i].trim(); // Удаляем лишние пробелы
        if (cookie.startsWith(name + "=")) { // Проверяем, начинается ли текущая куки с искомого имени
            return cookie.substring(name.length + 1); // Возвращаем значение куки (без имени)
        }
    }
    return null; // Если куки с указанным именем не найдено
}

function deleteCookie(name) {
    document.cookie = name + "=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
}

function setCookie(name, value, days) {
    var expires = "";
    if (days) {
        var date = new Date();
        date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
        expires = "; expires=" + date.toUTCString();
    }
    document.cookie = name + "=" + value + expires + "; path=/";
}

async function getQueryParam(parameterName) {
    // Получаем параметры строки запроса URL текущей страницы
    const queryString = window.location.search;

    // Создаем объект URLSearchParams из строки запроса
    const urlParams = new URLSearchParams(queryString);

    // Получаем значение параметра по его имени
    return urlParams.get(parameterName);
}