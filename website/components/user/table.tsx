import styled from "styled-components"

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

  td:first-child {
    width: 70%;
  }

  td:last-child {
    width: 10%;
  }


  a {
    text-decoration: none;
  }

  a:hover {
    cursor: pointer;
  }
`

export default Table
