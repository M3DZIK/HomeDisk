import Head from 'next/head'
import { useEffect, useState } from 'react'
import { useCookies } from 'react-cookie'
import styled from 'styled-components'
import { links } from '../config'

const Title = styled.h1`
  margin: 0;
  line-height: 1.15;
  font-size: 4rem;
  text-align: center;

  a {
    color: ${({ theme }) => theme.pages.index.title.a};
    text-decoration: none;
    animation: animate 1.5s linear infinite;
  }

  @keyframes animate {
     0% {
       opacity: 0;
     }
    50% {
      opacity: 1;
    }
    100% {
      opacity: 0;
    }
  }

  a:hover,
  a:focus,
  a:active {
    text-decoration: underline;
  }
`

const Description = styled.p`
  line-height: 1.5;
  font-size: 1.5rem;
  text-align: center;
`

const Grid = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  flex-wrap: wrap;
  max-width: 800px;
`

const Card = styled.div`
  margin: 1rem;
  padding: 1.5rem;
  text-align: left;
  color: inherit;
  text-decoration: none;
  border: 1px solid #eaeaea;
  border-radius: 10px;
  transition: color 0.15s ease, border-color 0.15s ease;
  max-width: 300px;

  :hover,
  :focus,
  :active {
    color: #0070f3;
    border-color: #0070f3;
  }

  h2 {
    margin: 0 0 1rem 0;
    font-size: 1.5rem;
  }

  p {
    margin: 0;
    font-size: 1.25rem;
    line-height: 1.5;
  }
`

const CardSignIn = styled.a`
  color: ${({ theme }) => theme.pages.index.cards.signin};
`

const CardRegister = styled.a`
  color: ${({ theme }) => theme.pages.index.cards.register};
`

const CardFiles = styled.a`
  color: ${({ theme }) => theme.pages.index.cards.files};
`

const CardSettings = styled.a`
  color: ${({ theme }) => theme.pages.index.cards.settings};
`

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
