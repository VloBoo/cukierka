const width_threshold = 480;

async function drawLineChart() {
  if ($("#lineChart").length) {
    ctxLine = document.getElementById("lineChart").getContext("2d");

    // Fetch data for line chart
    sendSql("SELECT * FROM MonthlyVacancies;").then(function (response) {
      console.log(response)

      const vacancyHits = [];
      const labels = [];

      response.rows.forEach(row => {
        vacancyHits.push(row.vacancy_count);
        labels.push(row.month);
      });

      const formattedDates = [];

      for (const dateString of labels) {
        // Преобразуем строку даты в объект Date
        const date = new Date(dateString);

        // Извлекаем год и месяц
        const year = date.getFullYear();
        const month = date.getMonth() + 1;

        // Формируем строку с годом и месяцем
        const formattedDate = `${year}-${month.toString().padStart(2, '0')}`;

        // Добавляем форматированную дату в массив
        formattedDates.push(formattedDate);
      }

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
          labels: formattedDates,
          datasets: [
            {
              label: "Количество вакансии",
              data: vacancyHits,
              fill: false,
              borderColor: "rgb(75, 192, 192)",
              lineTension: 0.1
            },
          ]
        },
        options: optionsLine
      };

      lineChart = new Chart(ctxLine, configLine);
    });;
  }
}

async function drawBarChart() {
  if ($("#barChart").length) {
    ctxBar = document.getElementById("barChart").getContext("2d");

    // Fetch data for bar chart
    sendSql("SELECT * FROM VacancyResponses;").then(function (response) {
      console.log(response);

      const skillLabels = [];
      const skillCounts = [];

      response.rows.forEach(row => {
        skillLabels.push(row.vacancy_title);
        skillCounts.push(row.response_count);
      });

      optionsBar = {
        responsive: true,
        scales: {
          xAxes: [
            {
              ticks: {
                display: false // Hide X-axis labels
              }
            }
          ],
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
              label: "Количество откликов",
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
    });
  }
}

async function drawPieChart() {
  if ($("#pieChart").length) {
    ctxPie = document.getElementById("pieChart").getContext("2d");

    // Fetch data for pie chart
    sendSql("SELECT * FROM VacancyStatusDistribution;").then(function (response) {
      console.log(response);

      const data_count = []
      const labels = [];

      response.rows.forEach(row => {
        data_count.push(row.status_count);
        labels.push(row.status);
      });

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
    });
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
