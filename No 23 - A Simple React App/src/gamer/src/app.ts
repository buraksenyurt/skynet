/*
    Sunucunun uygulamanın başlangıç noktası
*/
import express, { Express } from "express"
import mongoose from "mongoose"
//import cors from "cors"
import router from "./routes"
import bodyParser from "body-parser"

const app: Express = express()

const PORT: string | number = process.env.PORT || 5556

//app.use(cors()) // Cross Origin tanımlamaları için
app.use(router) // Http taleplerinin yönlendirmek için
app.use(bodyParser.urlencoded({extended:true}))

const uri: string = "mongodb://localhost:27017" //MongoDb bağlantı adresi
const options = { useNewUrlParser: true, useUnifiedTopology: true }
mongoose.set("useFindAndModify", false)

// MongoDb'ye belirtilen ayarlarla bağlandıktan
mongoose
  .connect(uri, options)
  .then(() => // sonra
    app.listen(PORT, () => // sunucu uygulama belirlenen porttan dinlemeye başlıyor
      console.log(`Sunucu uygulama http://localhost:${PORT} portunda etkin.`)
    )
  )
  .catch(error => {
    throw error
  })
