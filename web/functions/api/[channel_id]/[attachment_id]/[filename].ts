import { PagesFunction } from "@cloudflare/workers-types";

export const onRequest: PagesFunction = async ({ params }) => {
    let request = await fetch(`https://cdn.discordapp.com/attachments/${params.channel_id}/${params.attachment_id}/${params.filename}`);
    let response = await request.blob();
 	return new Response(response);
}
