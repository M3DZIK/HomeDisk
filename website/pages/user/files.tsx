import { faFile, faFolder } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import { Link as MuiLink } from "@mui/material"
import Head from 'next/head'
import Link from 'next/link'
import { useEffect, useState } from 'react'
import { useCookies } from 'react-cookie'
import styled from 'styled-components'
import api from '../../api'

const Table = styled.table`
  border: 1px solid;
  width: 80vw;
  border-collapse: collapse;

  thead {
    background-color: #01754b;
    color: white;
  }

  tr,
  td {
    border: 1px solid #000;
    padding: 8px;
  }

  a {
    text-decoration: none;
  }

  a:hover {
    cursor: pointer;
  }
`

const Icon = styled(FontAwesomeIcon)`
  padding-right: 5px;
  padding-left: 5px;
`

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

  useEffect(() => {
    const params = new URLSearchParams(window.location.search)
    const path = params.get("dir") || ""

    api.list(path, cookies.token)
      .then(data => {
        setPath(path)
        setFolders(data.dirs)
        setFiles(data.files)
      })
      .catch(err => console.error(err))
  }, [cookies])

  return (
    <>
      <Head>
        <title>Files - HomeDisk</title>
      </Head>

      <Table>
        <thead>
          <tr>
            <td>Name</td>
            <td>Size</td>
            <td>Modified</td>
          </tr>
        </thead>

        <tbody>
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
        <Link href={`?dir=${path}`}>
          <MuiLink onClick={() => refresh(path)}>
            <Icon icon={faFolder} />
            {name}
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
        <Link href={`?dir=${path}`}>
          <MuiLink onClick={() => refresh(path)}>
            <Icon icon={faFile} />
            {name}
          </MuiLink>
        </Link>
      </td>
      <td>{size}</td>
      <td>{modified}</td>
    </tr>
  )
}

type Props = {
  name: string,
  path: string,
  size: string,
  modified: string,
  refresh: (path: string) => any
}
