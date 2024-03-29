const paste = document.getElementById("paste");
const hashData = location.hash.replace("#", "");

async function replacePaste() {
  let resp = await fetch("https://pastebot-api.valk.sh/attachment" + hashData);
  let ctype = "utf-8";
  const maybe_ctype_raw = resp.headers.get("Content-Type");
  if (maybe_ctype_raw) {
    const maybe_ctype = maybe_ctype_raw.split("charset=", 2);
    if (maybe_ctype && maybe_ctype.length >= 2) {
      ctype = maybe_ctype[1].trim();
    }
  }
  const decoder = new TextDecoder(ctype);
  const reader = resp.body.getReader();
  while (true) {
    const { done, value } = await reader.read();
    if (done) {
      break;
    }
    const text = decoder.decode(value);
    paste.append(text);
  }
}
replacePaste().then(() => {});
