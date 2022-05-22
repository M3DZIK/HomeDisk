import styled from "styled-components"

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

export { Card, CardSignIn, CardRegister, CardFiles, CardSettings }
