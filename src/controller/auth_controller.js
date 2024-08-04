import { invoke } from "@tauri-apps/api/tauri";
import { createErrorToast, createSuccessToast } from "../toast.js";
import { listen } from "@tauri-apps/api/event";

export async function isValidSession() {
    if (document.cookie === "") {
        return false;
    }

    return await invoke("is_valid_session", {
        token: document.cookie.split("=")[1]
    });
}

export async function submitLoginForm(event) {
    event.preventDefault();
    const loginEmail = document.getElementById("login-modal-email"),
        loginPassword = document.getElementById("login-modal-password"),
        rememberMe = document.getElementById("login-modal-remember-me")

    await invoke("attempt_login", {
        email: loginEmail.value,
        password: loginPassword.value,
        remember: rememberMe.checked
    })
        .then(_ => {
            console.log("Now relocating to homepage");
            window.location.href = "../index.html"
        })
        .catch(e => {
            console.log("Hmm, seems like an error occured...")
            createErrorToast(e);
        })
}

export async function submitSignupForm(event) {
    event.preventDefault();
    const loginUsername = document.getElementById("signup-modal-username"),
        loginEmail = document.getElementById("signup-modal-email"),
        loginPassword = document.getElementById("signup-modal-password"),
        rememberMe = document.getElementById("signup-modal-remember-me")


    await invoke("attempt_signup", {
        username: loginUsername.value,
        email: loginEmail.value,
        password: loginPassword.value,
        remember: rememberMe.checked
    })
        .then(_ => {
            window.location.href = "../index.html";
        })
        .catch(e => createErrorToast(e))
}

listen('setJwtCookie', (event) => {
    console.log("Setting the cookie now");
    let expirationDate = new Date();
    expirationDate.setMonth(expirationDate.getMonth() + 1, expirationDate.getDay());
    document.cookie = `jwt=${event.payload}; expires=${expirationDate.toUTCString()}; path=/; Secure; SameSite=Strict`;
})
    .then(m => createSuccessToast(m))
    .catch(e => createErrorToast(e));

export function invalidateJwtCookie() {
    document.cookie = `jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;`;
}
