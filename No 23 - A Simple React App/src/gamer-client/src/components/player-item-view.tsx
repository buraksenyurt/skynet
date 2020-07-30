import React from "react"

type Props = PlayerProps & {
  deletePlayer: (_id: string) => void
}

const PlayerItemView: React.FC<Props> = ({ player, deletePlayer }) => {
  
  return (
    <div>
        <h1>{player.nickname}</h1>
        <p>{player.country} katılıyor. Güncel Puanı <b>({player.level})</b> <button onClick={() => deletePlayer(player._id)}>Çıkart</button> </p>
        
    </div>
  )
}

export default PlayerItemView