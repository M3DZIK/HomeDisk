import { Button } from '@mui/material'
import Head from 'next/head'
import { useCookies } from 'react-cookie'
import styled from 'styled-components'
import { links } from '../config'
import ThemeInterface from '../types/theme'

const Title = styled.h1`
  margin: 0;
  line-height: 1.15;
  font-size: 1.5rem;
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

const StyledButton = styled(Button)`
  margin-top: 1rem;
`

export default function NotFound() {
  return (
    <>
      <Head>
        <title>404 - HomeDisk</title>
      </Head>

      <Title>
        404 | This page could not be found.
      </Title>

      <StyledButton href="/">
        Go to Home Page
      </StyledButton>
    </>
  )
}
