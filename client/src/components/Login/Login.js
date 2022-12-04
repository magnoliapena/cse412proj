import './Login.css'
import { Link } from 'react-router-dom'
import { useState } from 'react'

const Login = () => {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')

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

    fetch('http://localhost:8080/api/login', request)
      .then(response => response.json())
      .then(data => console.log(data))
  }

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
