function confirm() {
    let name = document.getElementById("name").value;
    let token = getCookie("_t");
    if (token === undefined) {
        document.getElementById("errm").innerText = "Необходимо быть зарегистрированным!";
        document.getElementById("confirmbutton").disabled = true;
        return;
    }
    send("CreateProject", token, { name: name}).then((data) => {
        if (data.code == 200) {
            alert("Проект был создан");
            document.location = '/';
        } else {
            alert("Не удалось создать проект");
        }
    });
}