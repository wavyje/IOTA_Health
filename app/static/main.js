if (document.getElementById("scan")){
document.getElementById("scan").onclick = function () {
    location.href = "/scan";
};
}

if (document.getElementById("register")){
document.getElementById("register").onclick = function () {
    location.href = "/register";
};
}

if (document.getElementById("savedataonregister")) {
document.getElementById("savedataonregister").onclick = function () {
    location.href = "/home";
};
}

if (document.getElementById("officelogin")){
    document.getElementById("officelogin").onclick = function () {
        location.href = "/authenticate";
    };
    }

if (document.getElementById("authenticate")){
    let pw = document.getElementById('authenticate').value;
    /*document.getElementById("officelogin").onclick = function () {
        location.href = "/authenticate";
    };*/
    }


if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('static/service_worker.js')
    .then((reg) => console.log('Registered', reg))
    .catch((err) => console.log('Not registered', err))
    };