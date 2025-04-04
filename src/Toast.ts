import { listen } from "@tauri-apps/api/event";

let notifications = document.querySelector('.notifications');

function createToast(type: string, icon: string, title: string, text: string) {
	let newToast: any = document.createElement('div')
	newToast.innerHTML = `
            <div class="toast ${type}">
                <i class="${icon}"></i>
                <div class="content">
                    <div class="title">${title}</div>
                    <span>${text}</span>
                </div>
                <i id="notification-${notifications?.children.length}" class="fa-solid fa-xmark" onclick="(this.parentElement).remove()"></i>
            </div>`
	notifications?.appendChild(newToast)
	newToast.timeOut = setTimeout(() => newToast.remove(), 5000)
}

export function createSuccessToast(text: string) {
	let type = 'success';
	let icon = 'fa-solid fa-circle-check';
	let title = 'Success';
	if (!text.toString().includes("=>")) { // Filtering messages that are just lambda functions
		createToast(type, icon, title, text);
	}
}

export function createErrorToast(text: string) {
	let type = 'error';
	let icon = 'fa-solid fa-circle-exclamation';
	let title = 'Error';
	createToast(type, icon, title, text);
}

export function createWarningToast(text: string) {
	let type = 'warning';
	let icon = 'fa-solid fa-triangle-exclamation';
	let title = 'Warning';
	createToast(type, icon, title, text);
}

export function createInfoToast(text: string) {
	let type = 'info';
	let icon = 'fa-solid fa-circle-info';
	let title = 'Info';
	createToast(type, icon, title, text);
}

listen("createToast", (payload: any) => {
	switch (payload.payload[0]) {
		case "success":
			createSuccessToast(payload.payload[1]);
			break;
		case "error":
			console.log("Creating error toast now")
			createErrorToast(payload.payload[1]);
			break;
		case "info":
			createInfoToast(payload.payload[1]);
			break;
		case "warning":
			createWarningToast(payload.payload[1]);
			break;
	}
}).catch(e => console.log("Toast not created due to err: ", e))
