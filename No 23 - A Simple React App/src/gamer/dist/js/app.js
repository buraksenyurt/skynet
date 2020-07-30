"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
/*
    Sunucunun uygulamanın başlangıç noktası
*/
const express_1 = __importDefault(require("express"));
const mongoose_1 = __importDefault(require("mongoose"));
//import cors from "cors"
const routes_1 = __importDefault(require("./routes"));
const body_parser_1 = __importDefault(require("body-parser"));
const app = express_1.default();
const PORT = process.env.PORT || 5556;
//app.use(cors()) // Cross Origin tanımlamaları için
app.use(routes_1.default); // Http taleplerinin yönlendirmek için
app.use(body_parser_1.default.urlencoded({ extended: true }));
const uri = "mongodb://localhost:27017"; //MongoDb bağlantı adresi
const options = { useNewUrlParser: true, useUnifiedTopology: true };
mongoose_1.default.set("useFindAndModify", false);
// MongoDb'ye belirtilen ayarlarla bağlandıktan
mongoose_1.default
    .connect(uri, options)
    .then(() => // sonra
 app.listen(PORT, () => // sunucu uygulama belirlenen porttan dinlemeye başlıyor
 console.log(`Sunucu uygulama http://localhost:${PORT} portunda etkin.`)))
    .catch(error => {
    throw error;
});
