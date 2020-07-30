/*
    gamer isimli NodeJs sunucusu ile iletişim kuran proxy nesnemiz.
    Haberleşme için axios paketinden yararlanıyor
*/
import axios, { AxiosResponse } from "axios"

const baseUrl: string = "http://localhost:5556"

// Tüm oyuncu listeini çektiğimiz metot
export const getPlayers = async (): Promise<AxiosResponse<ApiMessage>> => {
  try {
    const players: AxiosResponse<ApiMessage> = await axios.get(
      baseUrl + "/players"
    )
    return players
  } catch (error) {
    throw new Error(error)
  }
}

// Yeni bir oyuncu eklemek için kullanılan metot
export const addPlayer = async (payload: IPlayer): Promise<AxiosResponse<ApiMessage>> => {
    try {
      const player: Omit<IPlayer, "_id"> = {
        nickname: payload.nickname,
        country: payload.country,
        level: payload.level,
      }
      const added: AxiosResponse<ApiMessage> = await axios.post(
        baseUrl + "/players",
        player
      )
      return added
    } catch (error) {
      throw new Error(error)
    }
  }

  // Silme operasyonu
  export const deletePlayer = async (_id: string): Promise<AxiosResponse<ApiMessage>> => {
    try {
      const deleted: AxiosResponse<ApiMessage> = await axios.delete(
        `${baseUrl}/players/${_id}`
      )
      return deleted
    } catch (error) {
      throw new Error(error)
    }
  }