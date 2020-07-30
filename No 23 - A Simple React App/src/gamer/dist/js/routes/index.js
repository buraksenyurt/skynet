"use strict";
/*
    Yönlendirici nesnemiz.
    İstemciden gelen HTTP taleplerini yol bilgisine göre alıp controller sınıfının ilgili metotlarına pas eder
*/
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = require("express");
const players_1 = require("../controllers/players");
const bodyParser = __importStar(require("body-parser"));
const router = express_1.Router();
router.use(bodyParser.json()); // Eğer bu satırı kullanmazsam, Post işlemi sonrasında body içeriğini alamıyorum. Undefined olarak dönüyor.
router.get("/players", players_1.getPlayers); // /players şeklinde gelen Get talepleri getPlayers metodunca karşılanır
router.post("/players", players_1.addPlayer); // Post talepleri add Player metoduna yönlendirilir
router.delete("/players/:id", players_1.deletePlayer); // players/{object_id} gibi id bilgisi ile birlikte gelen Http Delete talepleri de deletePlayer metoduna yönlendirilir
exports.default = router;
