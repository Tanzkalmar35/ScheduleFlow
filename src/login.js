import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";

let loginForm = document.getElementById("login-form");

loginForm.addEventListener("submit", submitLoginForm)

export async function submitLoginForm(event) {
    event.preventDefault();
    const loginUsername = document.getElementById("login-modal-username"),
        loginEmail = document.getElementById("login-modal-email"),
        loginPassword = document.getElementById("login-modal-password"),
        rememberMe = document.getElementById("login-modal-remember-me")


    await invoke("attempt_login", {
        username: loginUsername.value,
        email: loginEmail.value,
        password: loginPassword.value,
        remember: rememberMe.checked
    })
        .then(_ => console.log("Logged in successfully"))
        .catch(e => console.error(e))
}

listen('setJwtCookie', (event) => {
    document.cookie = `jwt=${event.payload}; path=/; Secure; SameSite=Strict`;
})
    .then(m => console.log(m))
    .catch(e => console.log(e));