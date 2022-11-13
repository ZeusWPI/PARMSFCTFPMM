const FLAG_URL = "http://localhost:4000"

const team_name_input = document.getElementById("team-name-input");
const flag_inputs = document.getElementsByClassName("flag-input");
const submit_btn = document.getElementById("submit-btn");

const flag_name_map = {};
for (const inp of flag_inputs) {
	const flag_name = inp.previousElementSibling.children[0].innerText;
	const flag_div = inp.parentElement.parentElement;

	flag_name_map[flag_name] = flag_div;
}

async function submit_flag(flag_input) {
	const flag_name = flag_input.previousElementSibling.children[0].innerText;

	const flag_value = flag_input.value;
	if (!(flag_value)) {
		alert("Missing flag value");
		return;
	}

	const team_name = team_name_input.value;
	if (!(team_name)) {
		alert("Missing team name");
		return;
	}

	localStorage.setItem("team_name", team_name);

	const res = await fetch(
		FLAG_URL + `/verify/${flag_name}/${flag_value}?team_name=${team_name}`,
		{
			method: "POST",
			mode: "cors"
		}
	);

	if (res.status == 403) {
		alert("You have already solved this flag");
		flag_input.value = "";

		return;
	} else if (res.status == 400) {
		alert("Incorrect team name");
		team_name_input.value = "";

		return;
	}

	const json = await res.json();
	const correct = json.correct;

	if (!(correct)) {
		alert("Incorrect!");
		flag_input.value = "";

		return;
	}

	alert("Correct!");
	flag_input.value = "";

	location.reload();
}

for (const inp of flag_inputs) {
	inp.addEventListener("keypress", async (e) => {
		if (e.key === "Enter") {
			await submit_flag(inp);
		}
	})
}

submit_btn.addEventListener("click", async (_) => {
	for (const inp of flag_inputs) {
		if (inp.value) {
			await submit_flag(inp);
			return;
		}
	}

	alert("Missing flag value");
})

function prefill_team_input() {
	const team_name = localStorage.getItem("team_name");

	if (team_name) {
		team_name_input.value = team_name;
	}
}

async function mark_solved() {
	const team_name = localStorage.getItem("team_name");
	if (!(team_name)) {
		return;
	}

	const res = await fetch(FLAG_URL + `/${team_name}/solved`);
	const solved = await res.json();

	console.log(solved);

	for (const flag_name of solved) {
		flag_name_map[flag_name].classList.add("solved");
	}
}

window.onload = async () => {
	prefill_team_input();
	await mark_solved()
};
