async function sendSql(sql) {
    const response = await fetch("/api/sql", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ sql: sql })
    });
    const data = await response.json();
    return data;
}

window.onload = async function() {
    const response = await sendSql("SELECT * FROM Resumes");
    const resumes = response.rows;

    const tableBody = document.getElementById("resumesTableBody");

    resumes.forEach(resume => {
        const row = document.createElement("tr");
        row.innerHTML = `
            <td>${resume.id}</td>
            <td><input type="text" value="${resume.title}" id="title_${resume.id}"></td>
            <td><input type="text" value="${resume.body}" id="body_${resume.id}"></td>
            <td><input type="number" value="${resume.payment}" id="payment_${resume.id}"></td>
            <td><input type="text" value="${resume.skill.join(', ')}" id="skill_${resume.id}"></td>
            <td>${resume.created}</td>
            <td>
                <button onclick="updateResume('${resume.id}')">Update</button>
                <button onclick="deleteResume('${resume.id}')">Delete</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
};

async function updateResume(resumeId) {
    const title = document.getElementById(`title_${resumeId}`).value;
    const body = document.getElementById(`body_${resumeId}`).value;
    const payment = document.getElementById(`payment_${resumeId}`).value;
    const skill = document.getElementById(`skill_${resumeId}`).value.split(', ');

    const sql = `UPDATE Resumes SET title='${title}', body='${body}', payment=${payment}, skill='{${skill}}' WHERE id='${resumeId}'`;
    await sendSql(sql);
    alert("Resume updated successfully!");
}

async function deleteResume(resumeId) {
    const confirmDelete = confirm("Are you sure you want to delete this resume?");
    if (confirmDelete) {
        const sql = `DELETE FROM Resumes WHERE id='${resumeId}'`;
        await sendSql(sql);
        alert("Resume deleted successfully!");
        // Reload the page after deletion
        location.reload();
    }
}
