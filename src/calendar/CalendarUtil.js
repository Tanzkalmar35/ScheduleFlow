import { invoke } from "@tauri-apps/api/tauri";
import { createErrorToast } from "../Toast.js";
import { Calendar } from "./Calendar.js";

// script.js
const monthYearElement = document.getElementById('month-year');
const calendarDatesElement = document.getElementById('calendar-dates');
const navigateToNextMonthBtn = document.getElementById('next-month-btn');
const navigateToPrevMonthBtn = document.getElementById('prev-month-btn');
const selectCalendarDropdown = document.getElementById('select-calendar');

// Get current date
let currentDate = new Date();

let calendars = [];

export async function loadUserCalendarData() {
    await invoke("get_calendar_of_current_user")
        .then(cal => {
            calendars = Calendar.map(cal);
        })
        .catch(e => createErrorToast(e));

    for (const calendar of calendars) {
        const option = document.createElement("option");
        option.setAttribute("value", calendar.name);
        option.textContent = calendar.name;
        selectCalendarDropdown.appendChild(option);
    }
}

/**
*   Renders the calendar to the home page
*
*   @param {Date} date - The current date
*/
function renderCalendar(date) {
    const month = date.getMonth();
    const year = date.getFullYear();
    const daysInMonth = new Date(year, month + 1, 0).getDate();

    // Calculate the day of the week on which the month starts
    const firstDayOfMonth = new Date(year, month, 0);
    const firstDayOfWeek = firstDayOfMonth.getDay();

    // Clear previous dates
    calendarDatesElement.innerHTML = '';

    // Render blank cells for days before the first day of the month
    for (let i = 0; i < firstDayOfWeek; i++) {
        const blankCell = document.createElement('div');
        blankCell.classList.add('date');
        blankCell.classList.add('blank');
        calendarDatesElement.appendChild(blankCell);
    }

    // Render dates
    for (let i = 1; i <= daysInMonth; i++) {
        const dateElement = document.createElement('div');
        dateElement.classList.add('date');
        dateElement.textContent = i;
        calendarDatesElement.appendChild(dateElement);
    }

    // Update month and year
    monthYearElement.textContent = `${date.toLocaleString('default', { month: 'long' })} ${year}`;


}

// Render calendar for current date
renderCalendar(currentDate);

// Render next month
navigateToNextMonthBtn.addEventListener("click", function() {
    currentDate = new Date(currentDate.getFullYear(), currentDate.getMonth() + 1);
    renderCalendar(currentDate);
});

// Render previous month
navigateToPrevMonthBtn.addEventListener("click", function() {
    currentDate = new Date(currentDate.getFullYear(), currentDate.getMonth() - 1);
    renderCalendar(currentDate);
});
