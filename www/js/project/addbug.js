function confirm() {
    let token = getCookie("_t");
    let pid = global['1'];

    let name = document.getElementById("name").value;
    let criticality = document.getElementById("criticality").value;
    let priority = document.getElementById("priority").value;
    let description = document.getElementById("description").value;


    send("CreateBug", token, { name: name, criticality: criticality, priority: priority, description: description, project: pid, user: token }).then((data) => {
        if (data.code == 200) {
            alert("Баг успешно добавлен");
            document.location = '../bugs/';
        } else {
            alert("Не удалось создать баг");
        }
    });
}