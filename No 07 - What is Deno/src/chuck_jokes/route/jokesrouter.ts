import { Router } from "https://deno.land/x/oak/mod.ts";
import {getAll} from '../controller/jokescontroller.ts';

const router=new Router();

router
    .get("/", getAll);

export default router;