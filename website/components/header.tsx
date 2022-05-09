import { faM, faMoon, faSun } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { RocketLaunch } from "@mui/icons-material";
import { AppBar, IconButton, Link, Stack, Toolbar, Typography } from "@mui/material";
import { useState } from "react";

export default function Footer({ toggleTheme, theme}: Props) {
  return (
    <AppBar
      position="static"
      sx={{ marginBottom: "calc(2% + 10px)" }}
    >
      <Toolbar>
        <Link href="/" color="inherit">
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="logo"
          >
            <RocketLaunch />
          </IconButton>
        </Link>

        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          HomeDisk
        </Typography>

        <Stack direction="row" spacing={2}>
          <div onClick={() => toggleTheme()}>
            <FontAwesomeIcon icon={theme == "light" ? faMoon : faSun} />
          </div>
        </Stack>
      </Toolbar>
    </AppBar>
  )
}

type Props = {
  toggleTheme: () => any,
  theme: string
}
