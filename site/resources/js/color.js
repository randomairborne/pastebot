function toggleColorMode() {
    if (document.getElementById("color").innerText === "Light mode") {
        setToLight();
    }
    else {
        setToDark();
    }
}
function onLoad() {
    if (localStorage.getItem("color") === "light") {
        document.getElementById("color").innerText === "Dark mode";
        setToLight();
    } else {
        document.getElementById("color").innerText === "Light mode";
        setToDark();
    }
}

function setToDark() {
    document.documentElement.style.color = "#fff";
    document.documentElement.style.backgroundColor = "#000";
    document.getElementById("color").innerText = "Light mode";
    localStorage.setItem("color", "dark");
}

function setToLight() {
    document.documentElement.style.color = "#000";
    document.documentElement.style.backgroundColor = "#fff";
    document.getElementById("color").innerText = "Dark mode";
    localStorage.setItem("color", "light");
}