/*
    Model nesnemiz.
    Veritabanı ile etkileşeme geçecek diğer tiplerde IPlayer sözleşmesine uyan bu modeli kullanabileceğiz
*/
import { IPlayer } from "./../types/player"
import { model, Schema } from "mongoose"

const playerSchema: Schema = new Schema(
  {
    nickname: {
      type: String,
      required: true,
    },

    country: {
      type: String,
      required: true,
    },

    level: {
      type: Number,
      required: true,
    },
  },
  { timestamps: true }
)

export default model<IPlayer>("Player", playerSchema)
