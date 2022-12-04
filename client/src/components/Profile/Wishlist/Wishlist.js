import './Wishlist.css'
import { useMemo } from 'react'
import { useTable } from 'react-table'
import { useState, useEffect } from 'react'
import useUser from '../../../useUser'

const Wishlist = () => {
  const { user } = useUser()
  const [data, setData] = useState(null)

  useEffect(() => {
    const request = {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ userid: user.userId })
    }
  
    fetch(`http://localhost:8080/api/user/${user.userId}/wishlist`, request)
      .then(response => response.json())
      .then(data => {
        setData(data)
      })
  }, [])

  // const data = useMemo(
  //   () => [
  //     {
  //       course: 'AAA101',
  //       title: 'Lost Civilizations',
  //       number: '222222',
  //       seats: '0 out of 24'
  //     },
  //     {
  //       course: 'AAA101',
  //       title: 'Lost Civilizations',
  //       number: '222222',
  //       seats: '0 out of 24'
  //     },
  //     {
  //       course: 'AAA101',
  //       title: 'Lost Civilizations',
  //       number: '222222',
  //       seats: '0 out of 24'
  //     }
  //   ],
  //   []
  // )

  const columns = useMemo(
    () => [
      {
        Header: 'Course',
        accessor: 'course', // accessor is the "key" in the data
      },
      {
        Header: 'Title',
        accessor: 'title',
      },
      {
        Header: 'Number',
        accessor: 'number',
      },
      {
        Header: 'Open Seats',
        accessor: 'seats',
      }
    ],
    []
  )

  const {
    getTableProps,
    getTableBodyProps,
    headerGroups,
    rows,
    prepareRow,
  } = useTable({ columns, data })

  return (
    <table {...getTableProps()} style={{"overflow": "hidden"}}>
      <thead>
        {headerGroups.map(headerGroup => (
          <tr {...headerGroup.getHeaderGroupProps()}>
            {headerGroup.headers.map(column => (
              <th
                {...column.getHeaderProps()}
                style={{
                  borderBottom: 'solid 0px white',
                  paddingBottom: '20px'
                }}
              >
                {column.render('Header')}
              </th>
            ))}
          </tr>
        ))}
      </thead>
      <tbody {...getTableBodyProps()}>
        {rows.map(row => {
          prepareRow(row)
          return (
            <tr {...row.getRowProps()}>
              {row.cells.map(cell => {
                return (
                  <td
                    {...cell.getCellProps()}
                    style={{
                      padding: '20px',
                      borderBottom: '0px solid white'
                    }}
                  >
                    {cell.render('Cell')}
                    <div style={{"width": "200%", "height": "1px", "backgroundColor": "white", "marginTop": "40px"}} />
                  </td>
                )
              })}
            </tr>
          )
        })}
      </tbody>
    </table>
  )
}

export default Wishlist
