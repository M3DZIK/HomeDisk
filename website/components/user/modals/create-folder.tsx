import React, { useState } from 'react'
import { Backdrop, Box, Button, Fade, Link, Modal, TextField } from "@mui/material"
import { useCookies } from "react-cookie"
import api from '../../../api_utils'
import style from './style'

function CreateFolderModal({ open, setOpen, refresh }: Props) {
  const [name, setName] = useState("")

  const [cookies] = useCookies(["token"])

  const handleChange: React.ChangeEventHandler<HTMLInputElement> = event => {
    const value = event.target.value
    setName(value)
  }

  // handle click "Enter (Return)"
  const handleKeyPress = (event: React.KeyboardEvent) => {
    if (event.keyCode === 13 || event.which === 13 || event.charCode === 13) {
      handle()
    }
  }

  const handle = () => {
    setOpen(false)

    const request = api.createDir(name, cookies.token)

    request
      .then(refresh)
  }

  return (
    <Modal
      open={open}
      onClose={() => setOpen(false)}
      closeAfterTransition
      BackdropComponent={Backdrop}
      BackdropProps={{
        timeout: 500,
      }}
    >
      <Fade in={open}>
        <Box sx={style}>
          <TextField
            label="Folder"
            placeholder="Folder"
            margin="normal"
            value={name}
            onChange={handleChange}
            onKeyPress={handleKeyPress}
          />

          <Link>
            <Button variant="outlined" onClick={handle}>
              Create Folder
            </Button>
          </Link>
        </Box>
      </Fade>
    </Modal>
  )
}

export type Props = {
  open: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>,
  refresh: () => void,
}

export default CreateFolderModal
