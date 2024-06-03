
function navigateTo(url: string) {
    window.location.href = url;
}

function navigateToIndexPage() {
    navigateTo("/");
}

function navigateToLoginPage() {
    navigateTo("../src/login.html")
}

function submitLoginForm() {
    const loginUsername = document.getElementById("login-modal-username"),
        loginEmail = document.getElementById("login-modal-email"),
        loginPassword = document.getElementById("login-modal-password")

    let result = {
        username: loginUsername,
        email: loginEmail,
        password: loginPassword
    }
}
