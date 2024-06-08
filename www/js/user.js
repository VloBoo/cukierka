
async function my() {
    let token = getCookie("token");
    if (token === null) {
        window.location.href = "/login";
    }

    getTokenById(token).then(function (response) {
        if (response.status === "ok") {
            getUserById(response.token.user_id).then(function (response) {
                document.getElementById('name').value = response.user.name;
                document.getElementById('email').value = response.user.email;
                document.getElementById('information').value = response.user.information;
                document.getElementById('comments_link').href = '/comment/list?id=' + response.user.id
            });
        } else {
            window.location.href = "/login";
        }
    });
}

async function user() {
    getQueryParam('id').then(function (id) {
        getUserById(id).then(function (response) {
            document.getElementById('name').value = response.user.name;
            document.getElementById('email').value = response.user.email;
            document.getElementById('information').value = response.user.information;
            document.getElementById('comments_link').href = '/comment/list?id=' + response.user.id
        });
    })
}