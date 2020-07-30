import React, { useEffect, useState } from 'react'
import PlayerItemView from './components/player-item-view'
import PlayerAddView from './components/player-add-view'
import { getPlayers, addPlayer, deletePlayer } from './proxy'

const App: React.FC = () => {
  const [players, setPlayers] = useState<IPlayer[]>([])

  useEffect(() => {
    fetchPlayers()
  }, [])

  const fetchPlayers = (): void => {
    getPlayers()
    .then(({ data: { players } }: IPlayer[] | any) => setPlayers(players))
    .catch((err: Error) => console.log(err))
  }

 const handleAddPlayer = (e: React.FormEvent, formData: IPlayer): void => {
   e.preventDefault()
   addPlayer(formData)
   .then(({ status, data }) => {
    if (status !== 201) {
      throw new Error('Kaydetme işleminde hata oluştu.')
    }
    fetchPlayers()
  })
  .catch((err) => console.log(err))
}

  const handleDeletePlayer = (_id: string): void => {
    deletePlayer(_id)
    .then(({ status, data }) => {
        if (status !== 200) {
          throw new Error('Beklenmedik bir hata oluştu.')
        }
        fetchPlayers()
      })
      .catch((err) => console.log(err))
  }

  return (
    <main className='App'>
      <h1>Best Players Catalog</h1>
      <PlayerAddView savePlayer={handleAddPlayer} />
      {players.map((p: IPlayer) => (
        <PlayerItemView
          key={p._id}
          deletePlayer={handleDeletePlayer}
          player={p}
        />
      ))}
    </main>
  )
}

export default App