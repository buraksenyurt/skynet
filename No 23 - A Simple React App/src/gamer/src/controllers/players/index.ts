/*
    Controller tipimiz temel CRUD operasyonlarını barındıracak olan typescript sınıfımızdır.
    HTTP servis iletişimi için express modülünü kullanır.
*/
import { Response, Request } from "express"
import { IPlayer } from "./../../types/player"
import Player from "../../models/player"

// Tüm oyuncu listesini çeker
const getPlayers = async (req: Request, res: Response): Promise<void> => {
  try {
    const players: IPlayer[] = await Player.find()
    res.status(200).json({ players })
  } catch (error) {
    throw error
  }
}

// Yeni bir oyuncu eklemek için kullanılır
const addPlayer = async (req: Request, res: Response): Promise<void> => {
    try {

    console.log(req.body)
        // HTTP talebine ait gövdeden gerekli parametre değerleri IPlayer arayüzüne okunur
      const body = req.body as Pick<IPlayer, 'nickname' | 'country' | 'level'>
  
      // Bu değerlerden yararlanarak yeni bir Player nesnesi örneklenir
      const player: IPlayer = new Player({
        nickname: body.nickname,
        country: body.country,
        level: body.level,
      })
  
      // save metodu ile yeni Player mongodb'ye yazılır
      const newPlayer: IPlayer = await player.save()
  
      res
        .status(201)
        .json({ message: "Yeni oyuncu eklendi", addedPlayer: newPlayer })
    } catch (error) {
      throw error
    }
  }

  // Sistemden oyuncu silmek için kullanılır
  const deletePlayer = async (req: Request, res: Response): Promise<void> => {
    try {
      const deleted: IPlayer | null = await Player.findByIdAndRemove(
        req.params.id
      )
      res.status(200).json({message: "Oyuncu silindi",deletedPlayer:deleted})
    } catch (error) {
      throw error
    }
  }
  
  export { getPlayers, addPlayer, deletePlayer }
  
  