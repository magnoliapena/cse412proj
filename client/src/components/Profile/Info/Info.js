import './Info.css'
import useUser from '../../../useUser'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'

const Info = () => {
  const { user } = useUser()
  const [data, setData] = useState(null)
  const navigate = useNavigate()

  useEffect(() => {
    if(!user) navigate('/login')
    else {
      // const request = {
      //   method: 'POST',
      //   headers: { 'Content-Type': 'application/json' },
      //   body: { user_id: user.userid }
      // }
    
      fetch(`http://98.161.210.47:8080/api/user/${user.userid}`)
        .then(response => response.json())
        .then(data => {
          setData(data)
        })
    }
  }, [])

  console.log(data)
  
  return data ? (
    <div>
      <p>Location: {data?.location}</p>
      <p>Major: {data?.major}</p>
    </div>
  ) : (
    <div>Loading...</div>
  )
}

export default Info
