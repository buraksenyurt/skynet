import allJokes from '../data/jokesdb.ts';
import joke from '../model/joke.ts';

export const getAll=(context:any)=>{
    context.response.body=allJokes;
}