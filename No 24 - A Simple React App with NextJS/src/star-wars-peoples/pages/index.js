import Head from 'next/head'
import styles from '../styles/Home.module.css'

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
  console.log('starwars-peoples',peoples); //F12 ile console penceresinden bakılabilir
  
  return (
    <div className={styles.container}>
      <Head>
        <title>Star Wars İnsanları</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Tüm Karakterler
        </h1>

        <p className={styles.description}>
          Star Wars evrenindeki tüm karaktelerin temel bilgilerini bu listede bulabilirsiniz;)
        </p>

        
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
