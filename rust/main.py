from selenium import webdriver
from selenium.webdriver.common.keys import Keys
from time import sleep
from os import getenv

driver = webdriver.Firefox()
driver.get("https://play.picoctf.org/login")

sleep(1)

driver.find_element(value='username').send_keys(getenv('THE_USER', 'admin'))
driver.find_element(value='password').send_keys(getenv('THE_PASSWORD', 'admin'))
driver.find_element(value='password').send_keys(Keys.ENTER)

sleep(1.5)

driver.get("https://play.picoctf.org/classrooms/3432")

sleep(1)

driver.execute_script('''
var table = document.querySelector(".main-panel > .content >  .container > .row > .col > .card > .card-body > .row > div > table");
document.body.innerHTML = table.innerHTML;

/* Remove all vsg's */
document.querySelectorAll("svg").forEach(e => e.remove());

var entries = {};

document.querySelectorAll(".card").forEach(
	row => {
		var login = row.querySelector(".col-6").innerText;
		var points = row.querySelector(".col-5").innerText.split(" ")[0];

		entries[login] = points;
	}
);

console.log(entries);

document.body.innerHTML = "";
document.head.innerHTML = "";

document.head.innerHTML = "<link href='https://cdn.jsdelivr.net/npm/bootstrap@5.2.2/dist/css/bootstrap.min.css' rel='stylesheet' integrity='sha384-Zenh87qX5JnK2Jl0vWa8Ck2rdkQ2Bzep5IDxbcnCeuOxjzrPF/et3URy9Bv1WTRi' crossorigin='anonymous'>"

var mainDiv =  document.createElement("div");
mainDiv.classList.add("w-100");
mainDiv.classList.add("p-4");

table = document.createElement("table");
table.classList.add("table");

mainDiv.appendChild(table);

document.body.appendChild(mainDiv);

let tbody = document.createElement("tbody");
table.appendChild(tbody);

for (const [login, points] of Object.entries(entries)) {
	let tr = document.createElement("tr");
	tbody.appendChild(tr);
	let loginElement = document.createElement("td");
	let pointsElement = document.createElement("td");
	tr.appendChild(loginElement);
	tr.appendChild(pointsElement);
	loginElement.innerHTML = login;
	pointsElement.innerHTML = points;
}
''')

input(">")

driver.close()
