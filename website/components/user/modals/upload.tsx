import React, { useState } from 'react'
import { Backdrop, Box, Button, Fade, Link, Modal } from "@mui/material"
import { useCookies } from "react-cookie"
import { toast } from 'react-toastify'
import api from '../../../api_utils'
import style from './style'

function UploadModal({ open, setOpen, path, refresh }: Props) {
  const [file, setFile]: [FileList | null | undefined, React.Dispatch<React.SetStateAction<FileList | null | undefined>>] = useState()

  const [cookies] = useCookies(["token"])

  const onFileChange: React.ChangeEventHandler<HTMLInputElement> = event => {
    setFile(event.target.files)
    console.log(file)
  }

  const handleUpload = () => {
    const formData = new FormData()

    if (file == null || typeof file == "undefined") {
      return
    }

    formData.append(
      "file",
      file[0]
    )

    const filePath = `${path}/${file[0].name}`

    const request = api.upload(filePath, formData, cookies.token)

    toast.promise(
      request,
      {
        pending: 'Uploading file...',
        success: {
          delay: 500,
          render() {
            refresh()
            setOpen(false)

            return "File uploaded!"
          }
        },
        error: {
          delay: 500,
          render(err) {
            if (err.data.response?.data?.error_message) {
              return err.data.response.data.error_message.toString()
            } else {
              return err.data.toString()
            }
          }
        }
      }
    )
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
          <input type="file" onChange={onFileChange} />

          <Link>
            <Button variant="outlined" onClick={handleUpload}>
              Upload
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
  path: string,
  refresh: () => void,
}

export default UploadModal
