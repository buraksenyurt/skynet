"use strict";
/*
    Yönlendirici nesnemiz.
    İstemciden gelen HTTP taleplerini yol bilgisine göre alıp controller sınıfının ilgili metotlarına pas eder
*/
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = require("express");
const players_1 = require("../controllers/players");
const router = express_1.Router();
router.get("/players", players_1.getPlayers); // /players şeklinde gelen Get talepleri getPlayers metodunca karşılanır
router.post("/players/add", players_1.addPlayer); // players/add şeklinde gelen Post talepleri de add Player metoduna yönlendirilir
router.delete("/players/delete/:id", players_1.deletePlayer); // players/delete/12 gibi id bilgisi ile birlikte gelen Http Delete talepleri de deletePlayer metoduna yönlendirilir
exports.default = router;
