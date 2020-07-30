/*
    Document tipinden kalıt alınmış bir arayüz sözleşmesi tanımladık.
    Mongo tarafındaki dokümanın içeriğinin ne olacağını belirlediğimizi söyleyebiliriz.
    MongoDB tarafı ile konuşurken bu sözleşme devreye girecek.
*/
import { Document } from "mongoose"

export interface IPlayer extends Document {
  nickname: string
  country: string
  level: number
}
