import { Router } from "https://deno.land/x/oak/mod.ts"; //HTTP Server görevini üstlenecek typescript modülü
import {getAll} from '../controller/jokescontroller.ts';

const router=new Router();

// Root web adresine gelen ki(http://localhost:5555 oluyor) talepler için yönlendirme
router
    .get("/", getAll);

export default router;