function filllist() {
    let token = getCookie("_t");
    if (token === undefined) {
        let el = document.createElement("h3");
        el.textContent = "Список проектов пуст"
        document.getElementById("listproject").appendChild(el)
        document.getElementById("createproject").disabled = true;
        return;
    }
    send("GetProjects", token, {}).then((data) => {
        if (data.code == 200) {
            console.log(data)
            let arr = data.arr;
            if (arr.length == 0) {
                let el = document.createElement("h3");
                el.textContent = "Список проектов пуст"
                document.getElementById("listproject").appendChild(el)
                return;
            }
            arr.forEach(function (project) {
                var projectId = project.project_id;
                let el = document.createElement("div");
                el.className = "row";

                send("GetProjectInfo", token, { id: projectId }).then((data) => {
                    if (data.code == 200) {
                        el.innerHTML = `<div style="flex-grow: 1;">Проект: ${data.name}</div><button class="blue" onclick="document.location='/p/${projectId}/'">К проекту</button>`;
                        document.getElementById("listproject").appendChild(el)
                    }
                });

            });

        } else {
            let el = document.createElement("h3");
            el.textContent = "Не удалось получить список проектов"
            document.getElementById("listproject").appendChild(el)
        }
    });
}