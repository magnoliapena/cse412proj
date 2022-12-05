import './Wishlist.css'
import { useMemo } from 'react'
import { useTable } from 'react-table'
import { useState, useEffect } from 'react'
import useUser from '../../../useUser'

const Wishlist = () => {
  const { user } = useUser()
  const [results, setResults] = useState(null)

  useEffect(() => {
    const request = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ userid: user.userid })
    }
  
    fetch(`http://98.161.210.47:8080/api/user/get_wishlist`, request)
      .then(response => response.json())
      .then(data => {
        setResults(data)
      })
  }, [])

  const handleRemove = (classid, term) => {
    const requestData = {
      userid: user.userid,
      classid,
      term
    }

    const request = {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(requestData)
    }

    fetch('http://98.161.210.47:8080/api/user/delete_from_wishlist', request)
      .then(response => response.json())
      .then(resData => {
        const request = {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ userid: user.userid })
        }
      
        fetch(`http://98.161.210.47:8080/api/user/get_wishlist`, request)
          .then(response => response.json())
          .then(data => {
            setResults(data)
          })
      })
  }

  return (
    <div>
      {results && results.map((result, index) => {
        return (
          <div key={index} className='Row Search-Result Wishlist-Row'>
            <button className='Button' onClick={() => handleRemove(result.classid, result.term)}>Remove</button>
            <p>{result.course}</p>
            <p>{result.days}</p>
            <p>{result.starttime} - {result.endtime}</p>
            <p>{result.instructor}</p>
            <p>{result.location}</p>
          </div>
        )
      })}
    </div>
  )
}

export default Wishlist
