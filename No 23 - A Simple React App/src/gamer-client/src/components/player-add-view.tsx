import React, { useState } from 'react'

type Props = { 
  savePlayer: (e: React.FormEvent, data: IPlayer | any) => void 
}

const PlayerAddView: React.FC<Props> = ({ savePlayer }) => {
  const [formData, setFormData] = useState<IPlayer | {}>()

  const handleForm = (e: React.FormEvent<HTMLInputElement>): void => {
    setFormData({
      ...formData,
      [e.currentTarget.id]: e.currentTarget.value,
    })
  }

  return (
    <form onSubmit={(e) => savePlayer(e, formData)}>
      <table>
        <tr>
         <td><label htmlFor='nickname'>Nick Name</label></td>
            <td><input onChange={handleForm} type='text' id='nickname' /></td>
        </tr>    
        <tr>
            <td><label htmlFor='country'>Country</label></td>
            <td><input onChange={handleForm} type='text' id='country' /></td>
        </tr>
        <tr>
            <td><label htmlFor='level'>Level</label></td>
            <td><input onChange={handleForm} type='text' id='level' /></td>
        </tr>
        <tr>
            <td></td>
            <td><button disabled={formData === undefined ? true: false} >Ekle</button></td>
        </tr>
    </table>
      
    </form>
  )
}

export default PlayerAddView