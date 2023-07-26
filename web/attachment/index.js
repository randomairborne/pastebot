const paste = document.getElementById("paste");
const hashData = location.hash.replace("#", "");

async function replacePaste() {
  let resp = await fetch("/api" + hashData);
  let data = await resp.arrayBuffer();
  let ctype = "utf-8";
  let maybe_ctype = resp.headers.get("Content-Type").split("charset=")[1];
  if (maybe_ctype) {
    maybe_ctype.trim();
    ctype = maybe_ctype;
  }
  const decoder = new TextDecoder(ctype);
  const text = decoder.decode(data);
  console.log(text);
  paste.innerText = text;
}
replacePaste().then(() => {});


