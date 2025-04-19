import { getCurrentCalendar, loadUserCalendarData } from "./models/CalendarUtil.ts";
import { invalidateJwtCookie, isValidSession } from "./controller/AuthController.js";
import { invoke } from "@tauri-apps/api/core";
import { createErrorToast, createSuccessToast } from "./Toast.ts";

const calendarModalUserList = document.getElementById("userList");

async function init() {
	if (!(await isValidSession())) {
		window.location.href = "../src/html/login.html";
		return;
	}

	invoke("set_app_handle").catch((e) =>
		console.log("Error updating the app handle: ", e),
	);

	await loadUserCalendarData();
	await appendListeners();
}

async function appendListeners() {
	// Give all close modal buttons their functionality.
	var closeButtons = document.getElementsByClassName("close-btn");
	for (var i = 0; i < closeButtons.length; i++) {
		closeButtons[i].addEventListener("click", function() {
			var modal = this.parentNode.parentNode.parentNode;
			modal.style.display = "none";
			document.getElementById("modalOverlay").style.display = "none";
		});
	}

	// Close new calendar modal after calendar creation
	document.getElementById("create-calendar-btn").onclick = () => {
		document.getElementById("create-calendar-modal").style.display =
			"block";
	};

	// Open edit calendar modal
	document.getElementById("edit-calendar-btn").onclick = () => loadEditCalendarModal(getCurrentCalendar());
	// Logout btn
	document.getElementById("logout-btn").onclick = () => logout();
	// Store new calendar
	document.getElementById("store-new-calendar-btn").onclick = (e) => storeNewCalendar(e);
	// Add user to calendar
	document.getElementById("add-users-form").onsubmit = (e) => addNewUser(e);
}

await init()
	.then(() => createSuccessToast("Initialized successfully"))
	.catch((e) => console.log("Initialization failed: " + e));

////////////////////////////////////////////////////////////////

async function logout() {
	await invoke("logout", {
		token: document.cookie.split("=")[1],
	}).catch((e) => createErrorToast(e));

	invalidateJwtCookie();
	window.location.reload();
}

async function storeNewCalendar(e) {
	e.preventDefault();
	const name = document.getElementById("new-calendar-name").value;
	const usersElements = document.getElementById("calendar-users").children;
	const users = getEmailsOfCalendarUsers(usersElements);

	await invoke("store_new_calendar", {
		calendarName: name,
		usersEmails: users,
	});

	document.getElementById("create-calendar-modal").style.display = "none";

	await loadUserCalendarData()
}

async function addNewUser(e) {
	e.preventDefault();
	const userEmail = document.getElementById("userEmail").value;

	if (getEmailsOfCalendarUsers(calendarModalUserList.children).includes(userEmail)) {
		return;
	}

	let userExists = false;

	await invoke("user_exists", { email: userEmail })
		.then((res) => {
			userExists = res;
		})
		.catch((err) => {
			createErrorToast(err);
		});

	if (!userExists) {
		createErrorToast("User does not exist")
		return;
	}

	calendarModalUserList.appendChild(createNewCalendarUserListElement(userEmail));
}

function getEmailsOfCalendarUsers(calendarUserList) {
	const users = [];
	let i = 0;
	while (i < calendarUserList.length) {
		const userElement = calendarUserList[i];
		users.push(userElement.children[0].textContent);
		i++;
	}
	return users;
}

/**
 * Loads the edit calendar modal and fills it with data.
 *
 * @param {[Calendar]} calendar - The calendar to edit.
 */
function loadEditCalendarModal(calendar) {
	const selectedCalendarName = document.querySelector('#select-calendar option:checked').textContent;

	if (selectedCalendarName == "Please select...") return;

	document.getElementById("calendarName").value = selectedCalendarName;
	document.getElementById("edit-calendar-modal").style.display = "block";
	document.getElementById("modalOverlay").style.display = "block";

	if (calendar.getUsers().length == 0) {
		calendarModalUserList.appendChild(createDummyUserListElement());
		return;
	}

	for (const user of calendar.getUsers()) {
		calendarModalUserList.appendChild(createNewCalendarUserListElement(user.getEmail()))
	}
}

/**
 * Creates a dummy user list entry for the case of the calendar having no users assigned.
 *
 * @returns {[HTMLElement]} The Html list element to append.
 */
function createDummyUserListElement() {
	const element = document.createElement("li");
	element.textContent = "No Users added yet..."

	return element;
}

/**
 * Creates a new user list entry for the calendar edit modal.
 *
 * @param {[User]} user - The user to list.
 * @returns {[HTMLElement]} The Html list element to append.
 */
function createNewCalendarUserListElement(userEmail) {
	const element = document.createElement("li");

	const spanChild = document.createElement("span");
	spanChild.textContent = userEmail;

	const removeBtn = document.createElement("button");
	removeBtn.onclick = () => removeUser(element.parentNode.children, userEmail);
	removeBtn.textContent = "Remove";

	element.appendChild(spanChild);
	element.appendChild(removeBtn);

	return element;
}

/**
 * Removes a user from the calendar users list.
 *
 * @param {[any]} calendarUserList - A list of users of the calendar.
 * @param {[string]} email - The email of the user to remove.
 */
function removeUser(calendarUserList, email) {
	let i = 0;
	while (i < calendarUserList.length) {
		const userElement = calendarUserList[i];
		if (userElement.children[0].textContent == email) {
			userElement.remove()
		}
		i++;
	}
}
