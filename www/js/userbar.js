function userbar() {
    let token = getCookie("_t")
    if (token === undefined) {
        document.getElementById("username").innerText = "Пользователь: Гость";
        document.getElementById("logout").style.display = "none"

    } else {
        send("GetUserInfo", token, {}).then((data) => {
            if (data.code == 200) {
                document.getElementById("username").innerText = "Пользователь: " + data.username;
                document.getElementById("signin").style.display = "none"
                document.getElementById("signup").style.display = "none"
            } else {
                document.getElementById("username").innerText = "Ошибка";
            }
        });
    }
}

function exit() {
    deleteCookie("_t");
    document.location.reload();
}