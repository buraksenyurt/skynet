import allJokes from '../data/jokesdb.ts';
import joke from '../model/joke.ts';

// jokesdb.ts teki tüm içeriği döndüren fonksiyon. Tipik HTTP Get All.
export const getAll=(context:any)=>{
    context.response.body=allJokes; // HTTP cevabının gövdesine tüm şakaları içine JSON nesnesini basıyoruz
}