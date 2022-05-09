import { GitHub } from "@mui/icons-material"
import { IconButton } from "@mui/material"
import styled from "styled-components"
import { links } from "../config"

const StyledFooter = styled.footer`
  width: 100%;
  height: 100px;
  border-top: 1px solid ${({ theme }) => theme.footer.borderTop};
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: ${({ theme }) => theme.colors.background};
  color: ${({ theme }) => theme.colors.color};

  a {
    display: flex;
    justify-content: center;
    align-items: cente;
  }
`

export default function Footer() {
  return (
    <StyledFooter>
      <IconButton color="inherit">
        <a href={links.github} target="_blank" rel="noreferrer" color="inherit">
          <GitHub />
        </a>
      </IconButton>
    </StyledFooter>
  )
}
