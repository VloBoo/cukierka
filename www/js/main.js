async function checkAuth() {
    let token = getCookie("token");
    if (token === null) {
        window.location.href = "/login";
    }

    getTokenById(token).then(function (response) {
        if (response.status === "ok") {
        } else {
            //window.location.href = "/login";
        }
    });
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
    let token = getCookie("token");

    deleteToken(token).then(function (response) {
        console.log(response)
    });

    deleteCookie("token");
    window.location.href = "/login";
}

