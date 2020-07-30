/*
    Yönlendirici nesnemiz.
    İstemciden gelen HTTP taleplerini yol bilgisine göre alıp controller sınıfının ilgili metotlarına pas eder
*/

import { Router } from "express"
import { getPlayers, addPlayer,  deletePlayer } from "../controllers/players"

const router: Router = Router()

router.get("/players", getPlayers) // /players şeklinde gelen Get talepleri getPlayers metodunca karşılanır

router.post("/players/add", addPlayer) // players/add şeklinde gelen Post talepleri de add Player metoduna yönlendirilir

router.delete("/players/delete/:id", deletePlayer) // players/delete/12 gibi id bilgisi ile birlikte gelen Http Delete talepleri de deletePlayer metoduna yönlendirilir

export default router