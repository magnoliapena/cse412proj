import './Info.css'
import useUser from '../../../useUser'
import { useEffect, useState } from 'react'

const Info = () => {
  const { user } = useUser()
  const [data, setData] = useState(null)

  useEffect(() => {
    const request = {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ userid: user.userId })
    }
  
    fetch(`http://localhost:8080/api/user/${user.userId}`, request)
      .then(response => response.json())
      .then(data => {
        setData(data)
      })
  }, [])
  
  return (
    <div>
      <div className='Row'>
        <p>Email:</p>
        <p>{data?.email}</p>
      </div>
      <div className='Row'>
        <p>Location:</p>
        <p>{data?.location}</p>
      </div>
      <div className='Row'>
        <p>Major:</p>
        <p>{data?.major}</p>
      </div>
    </div>
  )
}

export default Info
