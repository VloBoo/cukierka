function sendApi(url, method, message, token = null) {
    const headers = {};
    if (token) {
        headers['Token'] = token;
    }else{
        headers['Token'] = getCookie('token');
    }
    return new Promise((resolve, reject) => {
        $.ajax({
            url: `/api/${url}`,
            method: method,
            contentType: "application/json",
            data: message ? JSON.stringify(message) : null,
            headers: headers
        }).done(function (response) {
            resolve(response);
        }).fail(function (xhr, status, error) {
            console.error(error);
            //window.location.href = "/error/503";
            reject(error);
        });
    });
}

function sendSql(sql) {
    return sendApi('sql/', 'POST', {
        sql: sql
    });
}

// User

function createUser(name, email, password, information) {
    return sendApi('user/', 'POST', {
        name: name,
        email: email,
        password: password,
        information: information
    });
}

function getUserById(id) {
    return sendApi(`user/${id}`, 'GET', null);
}

function updateUser(id, token, name, email, password, information) {
    return sendApi(`user/${id}`, 'PUT', {
        name: name,
        email: email,
        password: password,
        information: information
    }, token);
}

function deleteUser(id, token) {
    return sendApi(`user/${id}`, 'DELETE', null, token);
}

// Token

function createToken(email, password) {
    return sendApi('token/', 'POST', {
        email: email,
        password: password
    });
}

function getTokenById(id) {
    return sendApi(`token/${id}`, 'GET', null);
}

function deleteToken(token) {
    return sendApi('token/', 'DELETE', null, token);
}

// Vacancy

function createVacancy(token, title, information, payment, status) {
    return sendApi('vacancy/', 'POST', {
        title: title,
        information: information,
        payment: payment,
        status: status
    }, token);
}

function getVacancyById(id) {
    return sendApi(`vacancy/${id}`, 'GET', null);
}

function updateVacancy(id, token, title, information, payment, status) {
    return sendApi(`vacancy/${id}`, 'PUT', {
        title: title,
        information: information,
        payment: payment,
        status: status
    }, token);
}

function deleteVacancy(id, token) {
    return sendApi(`vacancy/${id}`, 'DELETE', null, token);
}

function searchVacancies(title, sortBy, order) {
    return sendApi('vacancy/search/', 'POST', {
        title: title,
        sort_by: sortBy,
        order: order
    });
}

// Response

function createResponse(token, vacancyId) {
    return sendApi('response/', 'POST', {
        vacancy_id: vacancyId
    }, token);
}

function getResponseById(id) {
    return sendApi(`response/${id}`, 'GET', null);
}

function deleteResponse(id, token) {
    return sendApi(`response/${id}`, 'DELETE', null, token);
}

// Project

function createProject(token, responseId, vacancyId) {
    return sendApi('project/', 'POST', {
        response_id: responseId,
        vacancy_id: vacancyId
    }, token);
}

function getProjectById(id, token) {
    return sendApi(`project/${id}`, 'GET', null, token);
}

function deleteProject(id, token) {
    return sendApi(`project/${id}`, 'DELETE', null, token);
}

// Message

function createMessage(token, projectId, content) {
    return sendApi('message/', 'POST', {
        project_id: projectId,
        content: content
    }, token);
}

function getMessageById(id, token) {
    return sendApi(`message/${id}`, 'GET', null, token);
}

function deleteMessage(id, token) {
    return sendApi(`message/${id}`, 'DELETE', null, token);
}

// Comment

function createComment(token, user_id, rate, content) {
    return sendApi('comment/', 'POST', {
        user_id: user_id,
        rate: rate,
        content: content
    }, token);
}

function getCommentById(id) {
    return sendApi(`comment/${id}`, 'GET', null);
}

function deleteComment(id, token) {
    return sendApi(`comment/${id}`, 'DELETE', null, token);
}
