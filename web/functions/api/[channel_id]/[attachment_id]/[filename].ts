import { PagesFunction } from "@cloudflare/workers-types";

export const onRequest: PagesFunction = async ({ params }) => {
  let response = await fetch(
    `https://cdn.discordapp.com/attachments/${params.channel_id}/${params.attachment_id}/${params.filename}`
  );
  let body = await response.blob();
    return new Response(body, {
    headers: { "Content-Type": response.headers.get("Content-Type") },
  });
};
