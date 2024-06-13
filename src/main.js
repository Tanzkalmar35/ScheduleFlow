import {invoke} from "@tauri-apps/api/tauri";

async function isValidSession() {
    let result;

    await invoke("is_valid_session", {
        token: document.cookie.split("=")[1]
    })
        .then(isValid => {
            result = isValid;
        })
    return result;
}

if (!await isValidSession()) {
    window.location.href = '../src/login.html'
}
