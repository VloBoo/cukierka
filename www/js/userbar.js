function userbar() {
    let token = getCookie("_t")
    if (token === undefined) {
        document.getElementById("username").innerText = "Привет Гость";
        document.getElementById("logout").disabled = true;

    } else {
        send("GetUserInfo", token, {}).then((data) => {
            if (data.code == 200) {
                document.getElementById("username").innerText = "Привет " + data.username + "!";
                document.getElementById("signin").disabled = true;
                document.getElementById("signup").disabled = true;
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