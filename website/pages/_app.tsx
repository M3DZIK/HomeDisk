import { ThemeProvider as MuiThemeProvider, createTheme as muiCreateTheme, PaletteMode } from '@mui/material'
import { useEffect, useState } from 'react'
import { useCookies } from 'react-cookie'
import { createGlobalStyle, ThemeProvider } from 'styled-components'
import Container from '../components/container'
import Footer from '../components/footer'
import Header from '../components/header'
import Main from '../components/main'

const GlobalStyle = createGlobalStyle`
  html,
  body {
    padding: 0;
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Oxygen,
      Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue, sans-serif;
  }

  a {
    color: inherit;
    text-decoration: none;
  }

  * {
    box-sizing: border-box;
  }
`

const lightTheme = {
  colors: {
    background: "#ffffff",
    color: "#000000",
    error: "#f85b5b"
  },
  pages: {
    index: {
      cards: {
        signin: "#0a60cf",
        register: "#a06800",
        files: "#3d8011",
        settings: "#75006f"
      },
      title: {
        a: "#a109c0"
      }
    }
  },
  footer: {
    borderTop: "#eaeaea"
  }
}

const darkTheme = {
  colors: {
    background: "#131212",
    color: "#ffffff",
    error: "#f85b5b"
  },
  pages: {
    index: {
      cards: {
        signin: "#0a60cf",
        register: "#a06800",
        files: "#54ad19",
        settings: "#c90dbf"
      },
      title: {
        a: "#a109c0"
      }
    }
  },
  footer: {
    borderTop: "#161616"
  }
}

export default function App({ Component, pageProps }) {
  const [cookies, setCookies] = useCookies(["theme"])

  const [theme, setTheme] = useState(lightTheme)
  const [themeName, setThemeName]: [PaletteMode, any] = useState("light")

  useEffect(() => {
    if (!cookies.theme) setCookies("theme", "light")

    if (cookies.theme == "dark"){
      setTheme(darkTheme)
      setThemeName("dark")
    }
  }, [setCookies, setTheme, setThemeName, cookies])

  const toggleTheme = () => {
    if (cookies.theme == "light") {
      setTheme(darkTheme)
      setThemeName("dark")

      setCookies("theme", "dark")
    }
    if (cookies.theme == "dark") {
      setTheme(lightTheme)
      setThemeName("light")

      setCookies("theme", "light")
    }
  }

  const muiThene = muiCreateTheme({
    palette: {
      mode: themeName,
    },
  })

  return (
    <>
      <GlobalStyle />

      <MuiThemeProvider theme={muiThene}>
        <ThemeProvider theme={theme}>
          <Container>
            <Header toggleTheme={toggleTheme} theme={themeName} />

            <Main>
              <Component {...pageProps} />
            </Main>

            <Footer />
          </Container>
        </ThemeProvider>
      </MuiThemeProvider>
    </>
  )
}
