import './CreateAccount.css'
import { useState, useEffect } from 'react'
import useUser from '../../useUser'
import { useNavigate } from "react-router-dom"

const CreateAccount = () => {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [email, setEmail] = useState('')
  const [major, setMajor] = useState('')
  const [location, setLocation] = useState('')
  const { user, setUser } = useUser()
  const navigate = useNavigate()

  const handleSubmit = () => {
    const formData = {
      username,
      password,
      email,
      major,
      location
    }

    const request = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(formData)
    }

    fetch('http://98.161.210.47:8080/api/create_account', request)
      .then(response => response.json())
      .then(resData => {
        setUser(resData)
      })
  }

  useEffect(() => {
    if(user) navigate('/profile/info')
  }, [user])

  return (
    <div className='CreateAccount'>
      <h1>Create Account</h1>
      <div className='Row'>
        <div className="Column">
          <label>Username</label>
          <input className='TextInput' onChange={event => setUsername(event.target.value)} />
        </div>
        <div className="Column">
          <label>Password</label>
          <input className='TextInput' type='password' onChange={event => setPassword(event.target.value)} />
        </div>
      </div>
      <div className='Row'>
        <div className="Column">
          <label>Email</label>
          <input className='TextInput' onChange={event => setEmail(event.target.value)} />
        </div>
        <div className="Column">
          <label>Major</label>
          <input className='TextInput' onChange={event => setMajor(event.target.value)} />
        </div>
      </div>
      <div className="Column">
        <label>Location</label>
        <select className='TextInput' onChange={event => setLocation(event.target.value)} >
          <option value=''>Select Location</option>
          <option value='Tempe'>Tempe</option>
          <option value='West'>West</option>
          <option value='Polytechnic'>Polytechnic</option>
          <option value='Downtown Phoenix'>Downtown Phoenix</option>
          <option value='Online'>Online</option>
        </select>
      </div>
      <button className='Button' onClick={handleSubmit}>Create Account</button>
    </div>
  )
}

export default CreateAccount
