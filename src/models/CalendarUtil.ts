import { invoke } from "@tauri-apps/api/core";
import { Calendar } from "./Calendar.js";
import { IDate } from "../utils/SimpleDate.js";
import { createErrorToast } from "../Toast.ts";

// script.js
const monthYearElement: any = document.getElementById("month-year");
const calendarDatesElement: any = document.getElementById("calendar-dates");
const navigateToNextMonthBtn: any = document.getElementById("next-month-btn");
const navigateToPrevMonthBtn: any = document.getElementById("prev-month-btn");
const selectCalendarDropdown: any = document.getElementById("select-calendar");

// Get current date
let currentDate: Date = new Date();

let calendars: Calendar[] = [];
let currentSelectedCalendar: Calendar;

export async function loadUserCalendarData() {
	await invoke("get_calendar_of_current_user")
		.then((cal: any) => {
			calendars = Calendar.map(cal);
		})
		.catch((e) => createErrorToast(e));

	// Add calendar names to dropdown menu
	let i = 0;
	while (i < calendars.length) {
		const calendar = calendars[i];
		const option = document.createElement("option");
		option.setAttribute("value", calendar.getName());
		option.textContent = calendar.getName();
		selectCalendarDropdown?.appendChild(option);
		i++;
	}
}

/**
 *   Renders the calendar to the home page
 *
 *   @param {Date} date - The current date
 */
function renderCalendar(date: Date) {
	const month = date.getMonth();
	const year = date.getFullYear();
	const daysInMonth = new Date(year, month + 1, 0).getDate();

	// Calculate the day of the week on which the month starts
	const firstDayOfMonth = new Date(year, month, 0);
	const firstDayOfWeek = firstDayOfMonth.getDay();

	// Clear previous dates
	calendarDatesElement!.innerHTML = "";

	// Render blank cells for days before the first day of the month
	for (let i = 0; i < firstDayOfWeek; i++) {
		const blankCell = document.createElement("div");
		blankCell.classList.add("date");
		blankCell.classList.add("blank");
		calendarDatesElement?.appendChild(blankCell);
	}

	// Render dates
	for (let i = 1; i <= daysInMonth; i++) {
		const dateElement: any = document.createElement("div");
		dateElement.classList.add("date");
		dateElement.value = i;
		dateElement.textContent = i;
		calendarDatesElement?.appendChild(dateElement);
	}

	// Update month and year
	monthYearElement!.textContent = `${date.toLocaleString("default", { month: "long" })} ${year}`;
}

// Appends data of the currently selected calendar into the calendar gui.
function validateCalendarAndAppendUserData() {
	const selectedCalendarName = selectCalendarDropdown?.value;
	//const calendarDateElements = calendarDatesElement.children;

	// No calendar selected
	if (selectedCalendarName === "") {
		return;
	}

	const selectedCalendar = calendars.find(
		(cal: Calendar) => cal.getName() === selectedCalendarName,
	);

	if (!selectedCalendar) {
		console.error("Invalid calendar selected: " + selectedCalendar);
		return;
	}

	appendUserDataToCalendar(selectedCalendar);
	currentSelectedCalendar = selectedCalendar;
}

/**
 * Does the actual data appending to the calendar.
 *
 * @param {Calendar} selectedCalendar - The selected calendar
 */
function appendUserDataToCalendar(selectedCalendar: Calendar) {
	const amountOfComponents = selectedCalendar.getComponents().length;
	let i = 0;

	while (i < amountOfComponents) {
		let startDate = "";
		let endDate = "";
		const component = selectedCalendar.getComponents()[i];
		const entries = component.getProperties().entries();
		let iterator = entries.next();

		while (!iterator.done) {
			const [key, value] = iterator.value;

			if (key === "START_DATE") {
				startDate = value;
			} else if (key === "END_DATE") {
				endDate = value;
			}

			if (startDate !== "" && endDate !== "") {
				break;
			}

			iterator = entries.next();
		}

		// Converting plain string dates into Date objects
		const startDateObj = IDate.parseString(startDate);
		const endDateObj = IDate.parseString(endDate);

		let dateSpan = new Set();
		let tempDate = startDateObj;

		while (tempDate.compareTo(endDateObj) < 0) {
			dateSpan.add(tempDate);
			tempDate = tempDate.coypIncreaseOneDay();
		}

		// Get html elements where the date matches here
		const calendarDateElements = calendarDatesElement.children;
		const amountOfCalendarDateElements = calendarDateElements.length;
		let j = 0;
		// let affectedCalendarDateElements = new Set();

		while (j < amountOfCalendarDateElements) {
			const element = calendarDateElements[j];
			//const day = parseInt(element.textContent);

			dateSpan.forEach(_date => {
				element.textContent = "This is inside of a date span!";
				// affectedCalendarDateElements.push(element);
			})
			j++;
		}
		i++;
	}
}

export function getCurrentCalendar(): Calendar {
	return currentSelectedCalendar;
}

// Render calendar for current date
renderCalendar(currentDate);

// Render next month
navigateToNextMonthBtn.addEventListener("click", function() {
	currentDate = new Date(
		currentDate.getFullYear(),
		currentDate.getMonth() + 1,
	);
	renderCalendar(currentDate);
});

// Render previous month
navigateToPrevMonthBtn.addEventListener("click", function() {
	currentDate = new Date(
		currentDate.getFullYear(),
		currentDate.getMonth() - 1,
	);
	renderCalendar(currentDate);
});

selectCalendarDropdown.addEventListener("change", () => {
	validateCalendarAndAppendUserData()
});
