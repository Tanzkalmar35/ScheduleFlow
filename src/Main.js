import { getCurrentCalendar, loadUserCalendarData } from "./calendar/CalendarUtil.ts";
import { isValidSession } from "./controller/AuthController.js";
import { invoke } from "@tauri-apps/api/core";

async function init() {
	if (!(await isValidSession())) {
		window.location.href = "../src/html/login.html";
		return;
	}

	invoke("set_app_handle").catch((e) =>
		console.log("Error updating the app handle: ", e),
	);

	await loadUserCalendarData();

	// Give all close modal buttons their functionality.
	var closeButtons = document.getElementsByClassName("close-btn");
	for (var i = 0; i < closeButtons.length; i++) {
		closeButtons[i].addEventListener("click", function() {
			var modal = this.parentNode.parentNode.parentNode;
			modal.style.display = "none";
			document.getElementById("modalOverlay").style.display = "none";
		});
	}

	document.getElementById("create-calendar-btn").onclick = () => {
		document.getElementById("create-calendar-modal").style.display =
			"block";
	};

	document.getElementById("edit-calendar-btn").onclick = () => loadEditCalendarModal(getCurrentCalendar());
}

await init()
	.then(() => console.log("Initialized successfully"))
	.catch((e) => console.log("Initialization failed: " + e));

function loadEditCalendarModal(calendar) {
	const selectedCalendarName = document.querySelector('#select-calendar option:checked').textContent
	if (selectedCalendarName != "Please select...") {
		document.getElementById("calendarName").value = selectedCalendarName;
	} else {
		return;
	}
	document.getElementById("edit-calendar-modal").style.display =
		"block";
	document.getElementById("modalOverlay").style.display = "block";
}
