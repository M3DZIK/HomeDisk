import { Button, TextField } from '@mui/material'
import Head from 'next/head'
import Router from 'next/router'
import { useEffect, useState } from 'react'
import { useCookies } from 'react-cookie'
import styled from 'styled-components'
import api from '../api'

const Title = styled.h1`
  margin: 0;
  line-height: 1.15;
  font-size: 2rem;
  text-align: center;
  margin-bottom: 1rem;

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

const LoginButton = styled(Button)`
  margin-top: 1rem;
  align-content: "center";
`

const ErrorComponent = styled.div`
  color: ${({ theme }) => theme.colors.error};
`

export default function Login() {
  const [cookie, setCookie] = useCookies(["token"])

  useEffect(() => {
    if (cookie.token) {
      Router.push('/user/files')
    }
  })

  const [error, setError] = useState("")
  const [username, setUsername] = useState("")
  const [password, setPassword] = useState("")

  const handleUsernameChange: React.ChangeEventHandler<HTMLInputElement> = event => {
    const value = event.target.value
    setUsername(value)
  }

  const handlePasswordChange: React.ChangeEventHandler<HTMLInputElement> = event => {
    const value = event.target.value
    setPassword(value)
  }

  // handle click "Enter (Return)"
  const handleKeyPress = (event: React.KeyboardEvent) => {
    if (event.keyCode === 13 || event.which === 13 || event.charCode === 13) {
      handleLogin()
    }
  }

  const handleLogin = () => {
    const request = api.login(username, password)

    request
      .then(token => {
        setCookie("token", token)
        setError("")
      })
      .catch(err => setError(err.toString()))
  }

  return (
    <>
      <Head>
        <title>Login - HomeDisk</title>
      </Head>

      <Title>
        Sign in
      </Title>

      {error != "" && (
        <ErrorComponent>{error}</ErrorComponent>
      )}

      <TextField
        label="Username"
        placeholder="Username"
        margin="normal"
        value={username}
        onChange={handleUsernameChange}
        onKeyPress={handleKeyPress}
      />

      <TextField
        label="Password"
        placeholder="Password"
        type="password"
        margin="normal"
        value={password}
        onChange={handlePasswordChange}
        onKeyPress={handleKeyPress}
      />

      <LoginButton
        variant="contained"
        size="large"
        color="secondary"
        onClick={handleLogin}
        disabled={username == "" || password == ""}
      >
        Login
      </LoginButton>
    </>
  )
}
