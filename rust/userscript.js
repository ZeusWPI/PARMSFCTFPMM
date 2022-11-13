// ==UserScript==
// @name     Epic Leaderboard Scraper
// @version  1
// @match https://play.picoctf.org/classrooms/*
// @grant    GM.xmlHttpRequest
// ==/UserScript==

setTimeout(() => { /* Content is loaded dynamically, Wait for it to be loaded */
	let table = document.querySelector(".main-panel > .content >  .container > .row > .col > .card > .card-body > .row > div > table");
	document.body.innerHTML = table.innerHTML;


	/* Remove all vsg's */
	document.querySelectorAll("svg").forEach(e => e.remove());

	let entries = {};

	document.querySelectorAll(".card").forEach(
		row => {
			let login = row.querySelector(".col-6").innerText;
			entries[login] = row.querySelector(".col-5").innerText.split(" ")[0];
		}
	);

	GM.xmlHttpRequest({
		method: "POST",
		url: "http://localhost:8080/data",
		headers:    {
        	"Content-Type": "application/json"
    	},
		data: JSON.stringify(entries),
		onload: (e) => {}
	})
}, 1000);

setTimeout(() => {window.location.reload()}, 60000);
