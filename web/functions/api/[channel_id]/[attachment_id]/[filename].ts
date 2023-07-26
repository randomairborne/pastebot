import { PagesFunction } from "@cloudflare/workers-types";

export const onRequest: PagesFunction = async ({ params }) => {
  let url = `https://cdn.discordapp.com/attachments/${params.channel_id}/${params.attachment_id}/${params.filename}`;
  console.log(url);
  let response = await fetch(
    url,
    {
        headers: {
            "User-Agent": "pastebot/0.1 (+https://paste.valk.sh)",
            "Accept": "*/*"
        }
    }
  );
  let body = await response.arrayBuffer();
  console.log(JSON.stringify(Object.fromEntries(response.headers)));
  const dec = new TextDecoder("utf-8");
  console.log(dec.decode(body));
    return new Response(body, {
    headers: { "Content-Type": response.headers.get("Content-Type") },
  });
};
