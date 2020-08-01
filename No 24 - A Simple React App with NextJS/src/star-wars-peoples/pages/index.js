import Head from 'next/head'
import styles from '../styles/Home.module.css'

export default function Home() {
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
