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

// "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public';";
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

async function digestMessage(message) {
    const msgUint8 = new TextEncoder().encode(message); // encode as (utf-8) Uint8Array
    const hashBuffer = await crypto.subtle.digest("SHA-256", msgUint8); // hash the message
    const hashArray = Array.from(new Uint8Array(hashBuffer)); // convert buffer to byte array
    const hashHex = hashArray
        .map((b) => b.toString(16).padStart(2, "0"))
        .join(""); // convert bytes to hex string
    return hashHex;
}

async function getQueryParam(parameterName) {
    // Получаем параметры строки запроса URL текущей страницы
    const queryString = window.location.search;

    // Создаем объект URLSearchParams из строки запроса
    const urlParams = new URLSearchParams(queryString);

    // Получаем значение параметра по его имени
    return urlParams.get(parameterName);
}

/// ========================================================================================================================================
/// ============================================================== USER SPACE ==============================================================
/// ========================================================================================================================================

let user

async function checkAuth() {
    var login = getCookie("_e");
    var password = getCookie("_p");

    if (!login || !password) {
        window.location.href = "login.html";
    }

    result = await sendSql(`SELECT * FROM Users WHERE email = '${login}' and password = '${password}'`);
    if (result.error !== null && result.rows !== null && result.rows.length == 0) {
        window.location.href = "login.html";
    } else {
        user = result.rows[0];
        document.getElementById("nav-my-account").href = "account.html?id=" + user.id;
        console.log(document.getElementById("nav-my-account").href)
    }
}

async function login() {
    let email = document.getElementById("email").value;
    let password = await digestMessage(document.getElementById("password").value);

    let sqlQuery = `SELECT * FROM Users WHERE email = '${email}' AND password = '${password}';`;

    let result = await sendSql(sqlQuery);
    console.log(result)
    if (result.error !== null && result.rows !== null) {
        if (result.rows.length >= 1) {
            setCookie("_e", email, 30);
            setCookie("_p", password, 30);
            window.location.href = "index.html";
        }
        alert("Неверный логин или пароль. Перепроверьте данные.")
    } else {
        alert("Не удалось выполнить операцию. Перепроверьте данные или попробуйте позже.")
    }
}

async function userload() {
    let id = await getQueryParam('id')
    result = await sendSql(`SELECT * FROM Users WHERE id = '${id}'`);

    if (result.error !== null && result.rows !== null) {
        if (result.rows.length >= 1) {
            account = result.rows[0];
            console.log(account)
            document.getElementById("firstname").value = account.firstname;
            document.getElementById("secondname").value = account.secondname;
            document.getElementById("email").value = account.email;
            document.getElementById("usertype").value = account.type;
            document.getElementById("datecreated").value = account.created;
        } else {
            window.location.href = "404";
        }
    } else {
        alert("Не удалось выполнить операцию. Перепроверьте данные или попробуйте позже.")
    }

    let email = document.getElementById("email").value;
    let firstname = document.getElementById("firstname").value;
    let secondname = document.getElementById("secondname").value;
    let usertype = document.getElementById("usertype").value;
    let userId = await crypto.randomUUID();
    let currentDate = new Date().toISOString();
}

async function exit() {
    deleteCookie("_e");
    deleteCookie("_p");
    window.location.href = "login.html";
}

async function registration() {
    let email = document.getElementById("email").value;
    let firstname = document.getElementById("firstname").value;
    let secondname = document.getElementById("secondname").value;
    let usertype = document.getElementById("usertype").value;
    let password = await digestMessage(document.getElementById("password").value);
    let userId = await crypto.randomUUID();
    let currentDate = new Date().toISOString();

    let sqlQuery = `INSERT INTO Users (id, email, firstname, secondname, password, created, type) 
                    VALUES ('${userId}', '${email}', '${firstname}', '${secondname}', '${password}', '${currentDate}', '${usertype}');`;

    let result = await sendSql(sqlQuery);
    console.log(result)
    if (result.error !== null) {
        window.location.href = "login.html";
    } else {
        alert("Не удалось выполнить операцию. Перепроверьте данные или попробуйте позже.")
    }
}