import React from 'react'
import { TextField } from '@mui/material'
import Head from 'next/head'
import Router from 'next/router'
import { useEffect, useState } from 'react'
import { useCookies } from 'react-cookie'
import api from '../api_utils'
import ErrorComponent from '../components/auth/error'
import Title from '../components/auth/title'
import SubmitButton from '../components/auth/button'

export default function Login() {
  const [cookies, setCookies] = useCookies(["token"])

  useEffect(() => {
    if (cookies.token) {
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
    const request = api.register(username, password)

    request
      .then(token => {
        setCookies("token", token)
        setError("")
      })
      .catch(err => setError(err.toString()))
  }

  return (
    <>
      <Head>
        <title>Register - HomeDisk</title>
      </Head>

      <Title>
        Register
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

      <SubmitButton
        variant="contained"
        size="large"
        color="secondary"
        onClick={handleLogin}
        disabled={username == "" || password == ""}
      >
        Register
      </SubmitButton>
    </>
  )
}
