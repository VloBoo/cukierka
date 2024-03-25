const width_threshold = 480;

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


async function drawLineChart() {
  if ($("#lineChart").length) {
    ctxLine = document.getElementById("lineChart").getContext("2d");

    // Fetch data for line chart
    const lineChartData = await sendSql("SELECT * FROM CountVacanciesResponsesResumesByDay");

    const vacancyHits = [];
    const responseHits = [];
    const resumeHits = [];
    const labels = [];

    lineChartData.rows.forEach(row => {
      vacancyHits.push(row.vacancy_count);
      responseHits.push(row.response_count);
      resumeHits.push(row.resume_count);
      labels.push(row.day);
    });

    optionsLine = {
      scales: {
        yAxes: [
          {
            scaleLabel: {
              display: true,
              labelString: "Количество"
            }
          }
        ]
      }
    };

    // Set aspect ratio based on window width
    optionsLine.maintainAspectRatio = $(window).width() < width_threshold ? false : true;

    configLine = {
      type: "line",
      data: {
        labels: labels,
        datasets: [
          {
            label: "Вакансии",
            data: vacancyHits,
            fill: false,
            borderColor: "rgb(75, 192, 192)",
            lineTension: 0.1
          },
          {
            label: "Резюме",
            data: resumeHits,
            fill: false,
            borderColor: "rgba(255,99,132,1)",
            lineTension: 0.1
          },
          {
            label: "Откликов",
            data: responseHits,
            fill: false,
            borderColor: "rgba(99,255,132,1)",
            lineTension: 0.1
          }
        ]
      },
      options: optionsLine
    };

    console.log(configLine);

    lineChart = new Chart(ctxLine, configLine);
  }
}

async function drawBarChart() {
  if ($("#barChart").length) {
    ctxBar = document.getElementById("barChart").getContext("2d");

    // Fetch data for bar chart
    const barChartData = await sendSql("SELECT * FROM Top10PopularSkills");

    const skillLabels = [];
    const skillCounts = [];

    barChartData.rows.forEach(row => {
      skillLabels.push(row.skill);
      skillCounts.push(row.skill_count);
    });

    optionsBar = {
      responsive: true,
      scales: {
        yAxes: [
          {
            ticks: {
              beginAtZero: true
            },
            scaleLabel: {
              display: true,
              labelString: "Количество"
            }
          }
        ]
      }
    };

    optionsBar.maintainAspectRatio = $(window).width() < width_threshold ? false : true;

    configBar = {
      type: "bar",
      data: {
        labels: skillLabels,
        datasets: [
          {
            label: "Требуемые навыки",
            data: skillCounts,
            backgroundColor: [
              "rgba(255, 99, 132, 0.2)",
              "rgba(54, 162, 235, 0.2)",
              "rgba(255, 206, 86, 0.2)",
              "rgba(75, 192, 192, 0.2)",
              "rgba(153, 102, 255, 0.2)",
              "rgba(255, 159, 64, 0.2)"
            ],
            borderColor: [
              "rgba(255,99,132,1)",
              "rgba(54, 162, 235, 1)",
              "rgba(255, 206, 86, 1)",
              "rgba(75, 192, 192, 1)",
              "rgba(153, 102, 255, 1)",
              "rgba(255, 159, 64, 1)"
            ],
            borderWidth: 1
          }
        ]
      },
      options: optionsBar
    };

    barChart = new Chart(ctxBar, configBar);
  }
}

async function drawPieChart() {
  if ($("#pieChart").length) {
    ctxPie = document.getElementById("pieChart").getContext("2d");

    // Fetch data for pie chart
    const pieChartData = await sendSql("SELECT * FROM CountVacanciesResumes");

    const data_count = [pieChartData.rows[0].vacancy_count, pieChartData.rows[0].resume_count]
    const labels = ["Количество вакансий", "Количество резюме"];

    optionsPie = {
      responsive: true,
      maintainAspectRatio: false
    };

    configPie = {
      type: "pie",
      data: {
        datasets: [
          {
            data: data_count,
            backgroundColor: ["rgba(54, 162, 235, 0.4)", "rgba(255, 206, 86, 0.4)"],
            label: "All"
          }
        ],
        labels: labels
      },
      options: optionsPie
    };

    pieChart = new Chart(ctxPie, configPie);
  }
}

function updateChartOptions() {
  if ($(window).width() < width_threshold) {
    if (optionsLine) {
      optionsLine.maintainAspectRatio = false;
    }
    if (optionsBar) {
      optionsBar.maintainAspectRatio = false;
    }
  } else {
    if (optionsLine) {
      optionsLine.maintainAspectRatio = true;
    }
    if (optionsBar) {
      optionsBar.maintainAspectRatio = true;
    }
  }
}

function updateLineChart() {
  if (lineChart) {
    lineChart.options = optionsLine;
    lineChart.update();
  }
}

function updateBarChart() {
  if (barChart) {
    barChart.options = optionsBar;
    barChart.update();
  }
}

function reloadPage() {
  setTimeout(function () {
    window.location.reload();
  }); // Reload the page so that charts will display correctly
}
