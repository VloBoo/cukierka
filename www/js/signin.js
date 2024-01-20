function confirm() {
    let username = document.getElementById("username").value;
    let password = document.getElementById("password").value;
    send("CreateNewToken", null, { username: username, password: password }).then((data) => {
        if (data.code == 200) {
            if (data.result == "nope") {
                alert("Пароли не совпадают");
                return;
            }
            setCookie("_t", data.token);
            alert("Вы успешно зашли в аккаунт");
            document.location = '/';
        } else {
            alert("Не удалось зайти в аккаунт");
        }
    });

}