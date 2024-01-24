function bugslist() {

    let token = getCookie("_t");
    let pid = global['1'];

    send("GetBugs", token, { project: pid }).then((data) => {
        if (data.code == 200) {
            console.log(data)
            let arr = data.arr;
            if (arr.length == 0) {
                let el = document.createElement("h3");
                el.textContent = "Список багов пуст"
                document.getElementById("listbugs").appendChild(el)
                return;
            }

            var tableBody = document.querySelector("#data-table tbody");
            tableBody.innerHTML = "";

            arr.forEach(function (bug, i) {
                var bug = bug.id;
                send("GetBugInfo", token, { bug: bug }).then((data) => {
                    if (data.code == 200) {
                        var row = tableBody.insertRow();

                        var fieldsToDisplay = ["name", "description", "criticality", "priority", "status"];
                        fieldsToDisplay.forEach(function (field) {
                            var cell = row.insertCell();
                            cell.textContent = data[field];
                        });
                    }
                });
            });
        } else {
            let el = document.createElement("h3");
            el.textContent = "Не удалось получить список багов"
            document.getElementById("listbugs").appendChild(el)
        }
    });
}