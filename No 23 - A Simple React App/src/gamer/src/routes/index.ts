/*
    Yönlendirici nesnemiz.
    İstemciden gelen HTTP taleplerini yol bilgisine göre alıp controller sınıfının ilgili metotlarına pas eder
*/

import { Router } from "express"
import { getPlayers, addPlayer,  deletePlayer } from "../controllers/players"
import * as bodyParser from "body-parser"

const router: Router = Router()
router.use(bodyParser.json()) // Eğer bu satırı kullanmazsam, Post işlemi sonrasında body içeriğini alamıyorum. Undefined olarak dönüyor.

router.get("/players", getPlayers) // /players şeklinde gelen Get talepleri getPlayers metodunca karşılanır

router.post("/players", addPlayer) // Post talepleri add Player metoduna yönlendirilir

router.delete("/players/:id", deletePlayer) // players/{object_id} gibi id bilgisi ile birlikte gelen Http Delete talepleri de deletePlayer metoduna yönlendirilir

export default router