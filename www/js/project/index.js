function projectindex() {
    let token = getCookie("_t");
    let pid = global['1'];

    send("GetProjectInfo", token, { id: pid }).then((data) => {
        if (data.code == 200) {
            document.getElementById("projectname").innerText = data.name;
        } else {
            document.getElementById("projectname").innerText = "Ошибка";
            console.log(data);
        }
    });


    send("CheckPermission", token, { project: pid }).then((data) => {
        if (data.code == 200) {
            if (data.user_role == "owner") {
                document.getElementById("membersbutton").disabled = false;
                // document.getElementById("settingsbutton").disabled = false;
            }
        } else {
            console.log(data);
        }
    });
}