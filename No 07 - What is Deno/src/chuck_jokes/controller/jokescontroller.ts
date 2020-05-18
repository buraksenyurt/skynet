import { open, save } from "https://deno.land/x/sqlite/mod.ts"; //SQLite'ı işin içerisine katalım

//SQLite veritabanını bir hazırlayalım
const db = await open("ICNDB.db"); // International Chuck Norris Database :)
// Şayet veritabanı dosyasında Jokes tablosu yoksa oluşturalım
await db.query("CREATE TABLE IF NOT EXISTS Jokes (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT,popularity TEXT)")
await save(db);

// Yeni bir şaka eklemek için kullanılır
export const insert = async ({ request, response }: { request: any; response: any }) => {
    // HTTP Post Body'sinde bir şey yoksa 400 ile geri dönelim
    if (!request.hasBody) {
        response.status = 400;
        response.body = { msg: "Şaka mı yapıyorsun?" };
        return;
    }

    // body içeriğini content ve popularity'ye alalım. (bunların da olup olmadığını kontrol etmek lazım)
    const {
        value: { content, popularity }
    } = await request.body();

    //console.log(content + ' ' + popularity);

    // Insert sorgusunu çalıştıralım
    await db.query("INSERT INTO Jokes (content,popularity) VALUES (?,?)", [content, popularity]);
    // Değişikliği kaydedip kapatalım
    await save(db);

    // HTTP 200 OK dönelim
    response.status = 200;
    response.body = { message: 'Chuck Norris buna sevindi :D' }
}

// Select All fonksiyonu
export const getAll = async ({ request, response }: { request: any; response: any }) => {

    // Select sorgusunun sonucunu bir array'e alıyoruz
    const allJokes = [];
    for (const [id, content, popularity] of await db.query(
        "SELECT id,content,popularity FROM jokes ORDER BY id DESC")) {
        allJokes.push({ RuleNo: id, chuksMessage: content, Pop: popularity });
    }

    // HTTP 200 ile elde ettiğimiz içeriği döndürüyoruz
    response.status = 200;
    response.type = "application/json";
    response.body = JSON.stringify(allJokes);
}

//TODO: getById, Delete, Update gibi operasyonlar eklenebilir


// import allJokes from '../data/jokesdb.ts';
// import joke from '../model/joke.ts';

// // jokesdb.ts teki tüm içeriği döndüren fonksiyon. Tipik HTTP Get All.
// export const getAll=(context:any)=>{
//     context.response.body=allJokes; // HTTP cevabının gövdesine tüm şakaları içine JSON nesnesini basıyoruz
// }