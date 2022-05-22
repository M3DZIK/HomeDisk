import styled from "styled-components"

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

export default Title
