import Head from 'next/head'
import { useEffect, useState } from 'react'
import { useCookies } from 'react-cookie'
import { Card, CardFiles, CardRegister, CardSettings, CardSignIn } from '../components/home/cards'
import Description from '../components/home/description'
import Grid from '../components/home/grid'
import Title from '../components/home/title'
import { links } from '../config'

export default function Home() {
  const [cookies] = useCookies(["token"])

  const [cards, setCards] = useState(<CardsNonLogged />)

  useEffect(() => {
    if (cookies.token) {
      setCards(<CardsLogged />)
    }
  }, [setCards, cookies])

  return (
    <>
      <Head>
        <title>HomeDisk</title>
      </Head>

      <Title>
        Welcome to <a href={links.github} target="_blank" rel="noreferrer">HomeDisk!</a>
      </Title>

      <Description>
        Fast and lightweight local cloud for your data written in Rust
      </Description>

      <Grid>
        {cards}
      </Grid>
    </>
  )
}

function CardsNonLogged() {
  return (
    <>
      <Card>
        <CardSignIn href="/login">
          <h2>Sign in &rarr;</h2>
          <p>Log in to your account</p>
        </CardSignIn>
      </Card>

      <Card>
        <CardRegister href="/register">
          <h2>Register &rarr;</h2>
          <p>Register a new account</p>
        </CardRegister>
      </Card>
    </>
  )
}

function CardsLogged() {
  return (
    <>
      <Card>
        <CardFiles href="/user/files">
          <h2>Files &rarr;</h2>
          <p>View your files</p>
        </CardFiles>
      </Card>

      <Card>
        <CardSettings href="/user/settings">
          <h2>Settings &rarr;</h2>
          <p>Go to user settings</p>
        </CardSettings>
      </Card>
    </>
  )
}
