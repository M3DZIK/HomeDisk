import { faFile, faFolder } from "@fortawesome/free-solid-svg-icons"
import { CloudUpload } from "@mui/icons-material"
import { IconButton, Link as MuiLink } from "@mui/material"
import Head from 'next/head'
import Link from 'next/link'
import { useEffect, useState } from 'react'
import { useCookies } from 'react-cookie'
import { resolve as pathResolve } from 'path'
import api from '../../api_utils'
import Icon from "../../components/other/icon"
import Table from "../../components/user/table"
import UploadModal from "../../components/user/modals/upload"

export default function Files() {
  const [cookies] = useCookies(["token"])

  const [path, setPath] = useState("")
  const [files, setFiles] = useState([{ name: "", size: "", modified: "" }])
  const [folders, setFolders] = useState([{ name: "", size: "", modified: "" }])

  const refresh = (path: string) => {
    api.list(path, cookies.token)
      .then(data => {
        setPath(path)
        setFolders(data.dirs)
        setFiles(data.files)
      })
      .catch(err => console.error(err))
  }

  const refreshFolder = () => refresh(path)

  useEffect(() => {
    const params = new URLSearchParams(window.location.search)
    const path = params.get("path") || ""

    api.list(path, cookies.token)
      .then(data => {
        setPath(path)
        setFolders(data.dirs)
        setFiles(data.files)
      })
      .catch(err => console.error(err))
  }, [cookies])

  // modals
  const [uploadModal, setUploadModal] = useState(false)

  return (
    <>
      <Head>
        <title>Files - HomeDisk</title>
      </Head>

      <MuiLink onClick={() => setUploadModal(true)}>
        <IconButton
          size="large"
          edge="start"
          color="inherit"
          aria-label="logo"
        >
          <CloudUpload />
        </IconButton>
      </MuiLink>

      <UploadModal open={uploadModal} setOpen={setUploadModal} path={path} refresh={refreshFolder} />

      <Table>
        <thead>
          <tr>
            <td>Name</td>
            <td>Size</td>
            <td>Modified</td>
          </tr>
        </thead>

        <tbody>
          {path != "" && path != "/" && (
            <tr>
              <td>
                <Link href={`?path=${pathResolve(path, '..')}`}>
                  <MuiLink onClick={() => refresh(pathResolve(path, '..'))}>
                    <Icon icon={faFolder} />
                    .. (go up)
                  </MuiLink>
                </Link>
              </td>
              <td></td>
              <td></td>
            </tr>
          )}

          {folders.map((f, index) => <FolderComponent key={index} name={f.name} path={`${path}/${f.name}`} size={f.size} modified={f.modified} refresh={refresh} />)}
          {files.map((f, index) => <FileComponent key={index} name={f.name} path={`${path}/${f.name}`} size={f.size} modified={f.modified} refresh={refresh} />)}
        </tbody>
      </Table>
    </>
  )
}

function FolderComponent({ name, path, size, modified, refresh }: Props) {
  return (
    <tr>
      <td>
        <Link href={`?path=${path}`}>
          <MuiLink onClick={() => refresh(path)}>
            <Icon icon={faFolder} />
            {name.replace("/", "")}
          </MuiLink>
        </Link>
      </td>
      <td>{size}</td>
      <td>{modified}</td>
    </tr>
  )
}

function FileComponent({ name, path, size, modified, refresh }: Props) {
  return (
    <tr>
      <td>
        <Link href={`?path=${path}`}>
          <MuiLink onClick={() => refresh(path)}>
            <Icon icon={faFile} />
            {name.replace("/", "")}
          </MuiLink>
        </Link>
      </td>
      <td>{size}</td>
      <td>{modified} ago</td>
    </tr>
  )
}

type Props = {
  name: string,
  path: string,
  size: string,
  modified: string,
  // eslint-disable-next-line no-unused-vars
  refresh: (path: string) => void
}
