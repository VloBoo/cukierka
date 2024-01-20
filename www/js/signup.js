function confirm() {
    let username = document.getElementById("username").value;
    let email = document.getElementById("email").value;
    let password = document.getElementById("password").value;
    send("CreateUser", "null", { username: username, email: email, password: password }).then((data) => {
        if (data.code == 200) {
            alert("Вы успешно зарегистрировали аккаунт");
            document.location = '/account/signin/';
        } else {
            alert("Не удалось зарегистрировать аккаунт");
        }
    });
}