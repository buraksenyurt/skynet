import { Application } from "https://deno.land/x/oak/mod.ts";
import router from "./route/jokesrouter.ts";

const PORT = 3000;
const app = new Application();

app.use(router.routes());
app.use(router.allowedMethods());

console.log(`Chuck Norris ${PORT} nolu portta hazır :[] `);

await app.listen({ port: PORT });