import { Application } from "https://deno.land/x/oak/mod.ts"; //Modül sistemi NodeJS ile farklı. URL kullanılıyor.
import router from "./route/jokesrouter.ts"; // Deno Varsayılan olarak Typescript kullanıyor ve destekliyor
import errorHandler from "./errorHandler.js";
import { open, save } from "https://deno.land/x/sqlite/mod.ts";

const PORT = 5555;
const app = new Application();

app.use(errorHandler);
// Middleware'e router eklendi.
app.use(router.routes());
// HTTP Get, Post, Put, Delete, Head, Options, Patch metodlarının kullanımına izin veriyoruz
app.use(router.allowedMethods());

console.log(`Chuck Norris ${PORT} nolu portta hazır :[] `);

// Portu açıp dinlemeye başlıyoruz
app.listen({ port: PORT });