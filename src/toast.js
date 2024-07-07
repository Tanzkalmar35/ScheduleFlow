import {listen} from "@tauri-apps/api/event";

let notifications = document.querySelector('.notifications');
let loginForm = document.getElementById("login-btn");

function createToast(type, icon, title, text) {
    let newToast = document.createElement('div')
    newToast.innerHTML = `
            <div class="toast ${type}">
                <i class="${icon}"></i>
                <div class="content">
                    <div class="title">${title}</div>
                    <span>${text}</span>
                </div>
                <i class="fa-solid fa-xmark" onclick="(this.parentElement).remove()"></i>
            </div>`
    notifications.appendChild(newToast)
    newToast.timeOut = setTimeout(() => newToast.remove(), 5000)
}

export function createSuccessToast(text) {
    let type = 'success';
    let icon = 'fa-solid fa-circle-check';
    let title = 'Success';
    if (!text.toString().includes("=>")) { // Filtering messages that are just lambda functions
        createToast(type, icon, title, text);
    }
}

export function createErrorToast(text) {
    let type = 'error';
    let icon = 'fa-solid fa-circle-exclamation';
    let title = 'Error';
    createToast(type, icon, title, text);
}

export function createWarningToast(text) {
    let type = 'warning';
    let icon = 'fa-solid fa-triangle-exclamation';
    let title = 'Warning';
    createToast(type, icon, title, text);
}

export function createInfoToast(text) {
    let type = 'info';
    let icon = 'fa-solid fa-circle-info';
    let title = 'Info';
    createToast(type, icon, title, text);
}

listen("createToast", (type, text) => {
    switch (type) {
        case "success":
            createSuccessToast(text);
            break;
        case "error":
            createErrorToast(text);
            break;
        case "info":
            createInfoToast(text);
            break;
         case "warning":
            createWarningToast(text);
            break;
    }
})
    .then(r => {})
    .catch(e => {})
