async function login() {
    let email = document.getElementById("email").value;
    let password = document.getElementById("password").value;

    createToken(email, password, null).then(function (response) {
        if (response.status === "ok") {
            setCookie("token", response.token, 30);
            alert("Вы успешно вошли в аккаунт");
            window.location.href = "/";
        } else {
            console.warn(response.status);
            alert("Не удалось войти в аккаунт");
        }
    });
}

async function registration() {
    let name = document.getElementById("name").value;
    let email = document.getElementById("email").value;
    let password = document.getElementById("password").value;

    createUser(name, email, password).then(function (response) {
        if (response.status === "ok") {
            alert("Вы успешно зарегистрированы!");
            window.location.href = "login";
        } else {
            alert("Не удалось пройти регистрацию");
        }
    });
}