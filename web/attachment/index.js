const paste = document.getElementById("paste");
const hashData = location.hash.replace("#", "");

async function replacePaste() {
  let resp = await fetch("https://pastebot-api.valk.sh/attachment" + hashData);
  let ctype = "utf-8";
  let maybe_ctype = resp.headers.get("Content-Type").split("charset=")[1];
  if (maybe_ctype) {
    maybe_ctype.trim();
    ctype = maybe_ctype;
  }
  const decoder = new TextDecoder(ctype);
  for await (const chunk of resp.body) {
    const text = decoder.decode(data);
    console.log(text);
    paste.append(text);
  }
}
replacePaste().then(() => {});


