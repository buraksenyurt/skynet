interface IPlayer {
    _id: string
    nickname: string
    country: string
    level: number
    createdAt?: string
    updatedAt?: string
  }
  
  interface PlayerProps {
    player: IPlayer
  }
  
  type ApiMessage = {
    message: string
    status: string
    players: IPlayer[]
    player?: IPlayer
  }