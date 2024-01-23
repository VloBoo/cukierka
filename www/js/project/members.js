function memberslist() {

    let token = getCookie("_t");
    let pid = global['1'];

    send("GetMembers", token, { project: pid }).then((data) => {
        if (data.code == 200) {
            console.log(data)
            let arr = data.arr;
            if (arr.length == 0) {
                let el = document.createElement("h3");
                el.textContent = "Список проектов пуст"
                document.getElementById("listmembers").appendChild(el)
                return;
            }
            arr.forEach(function (members) {
                var members = members.user_id;
                let el = document.createElement("div");
                el.className = "row";

                send("GetUserInfoById", token, { user: members }).then((data) => {
                    if (data.code == 200) {
                        el.innerHTML = `<div>${data.username}</div>`;
                        document.getElementById("listmembers").appendChild(el)
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