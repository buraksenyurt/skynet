import Head from 'next/head'
import styles from '../styles/Home.module.css'
import {Card,ListGroup} from 'react-bootstrap'; // Bootstrap elemanlarını kullanabilmek için ekledik

const url=`https://swapi.dev/api/people/`; // Kullanacağımız servis adresini bir sabit değişkene aldık

/*
  üstte tanımladığımızı adresi kullanarak tüm JSON verisini çeken asenkron fonksiyonumuz.
  getServerSideProps, nextjs'in veri çeken fonksiyonlarından birisidir.
  Farklı isimle kullanmaya çalıştığımızda çağırılmadığını görürürüz. 
*/
export async function getServerSideProps(){
  const response=await fetch(url); // HTTP Get talebini gönderdik
  const peoples=await response.json(); // Çekilen veri JSON formatında olduğu için, dönüştürdük
  /*
    component üzerinde verinin kullanılabilmesi için,
    properties içerisine ekledik. Bunu Home bileşeninde kullanmak için
    parametre olarak eklediğimize dikkat edelim.
  */
  return{
    props:{
      peoples
    }
  }
}

export default function Home({peoples}) { //props ile gelen peoples parametre olarak eklendiği için içeride kullanılabilecek
  
  // İlk denemede verinin gelip gelmediğini tespit etmek için kullanabiliriz.
  // console.log('starwars-peoples',peoples); //F12 ile console penceresinden bakılabilir
  
  // İstediğimiz bilgiler JSON verisindeki results altında duruyor. Bunları bir diziye aldık.
  // Gir kontrolünün içeriğini doldururken bir array'den yararlanıyoruz.
  const results=[]=peoples.results;
  //console.log(results);

  return (
    <div className={styles.container}>
      <Head>
        <title>Star Wars İnsanları</title>
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Tüm Karakterler
        </h1>

        <p className={styles.description}>
          Star Wars evrenindeki tüm karaktelerin temel bilgilerini bu listede bulabilirsiniz;)
        </p>

        <ListGroup>
          {results.map(r=>{
            const {name,birth_year,height}=r;
            return(
              <Card styles={{width:'18rem'}}>
                <Card.Body>
                  <Card.Title>
                    {name}
                  </Card.Title>
                  <Card.Text>
                    {name}, {birth_year} yılında doğmuştur. Boyu {height} cm'dir.
                  </Card.Text>
                </Card.Body>
              </Card>
            )
          })}
        </ListGroup>
      </main>

      <footer className={styles.footer}>
        <a
          href="https://swapi.dev/"
          target="_blank"
          rel="noopener noreferrer"
        >Diğer API Hizmetleri için tıklayın.
        </a>
      </footer>
    </div>
  )
}
