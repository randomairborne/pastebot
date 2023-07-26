const paste = document.getElementById("paste");
const hashData = location.hash.replace("#", "");
fetch("/api" + hashData)
  .then((response) => response.text())
  .then(function (data) {
    paste.innerText = data;
  });