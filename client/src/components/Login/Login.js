import './Login.css'
import { Link } from 'react-router-dom'
import { useEffect, useState } from 'react'
import useUser from '../../useUser'
import { useNavigate } from "react-router-dom"

const Login = () => {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const { user, setUser } = useUser()
  const navigate = useNavigate()

  const handleSubmit = () => {
    const data = {
      username,
      password
    }

    const request = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data)
    }

    fetch('http://98.161.210.47:8080/api/login', request)
      .then(response => response.json())
      .then(resData => {
        setUser(resData)
      })
  }

  useEffect(() => {
    if(user) navigate('/profile/info')
  }, [user])

  return (
    <div className="Login">
      <h1>Log In</h1>
      <div className="Column">
        <label>Username</label>
        <input className="TextInput" onChange={event => setUsername(event.target.value)} />
      </div>
      <div className="Column">
        <label>Password</label>
        <input className="TextInput" type='password' onChange={event => setPassword(event.target.value)} />
      </div>
      <button className="Button" onClick={handleSubmit}>Log In</button>
      <div style={{ "display": "flex", "alignItems": "center", "justifyContent": "center" }}>
        <div style={{"width": "70px", "height": "2px", "backgroundColor": "white"}}></div>
        <div style={{ "margin": "0px 10px 8px 10px" }}>or</div>
        <div style={{"width": "70px", "height": "2px", "backgroundColor": "white"}}></div>
      </div>
      <button className="Button"><Link className='Link' style={{ "color": "black" }} to='/create'>Create Account</Link></button>
    </div>
  )
}

export default Login
