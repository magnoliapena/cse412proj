import './Login.css'
import { Link } from 'react-router-dom'

const Login = () => {
  return (
    <div className="Login">
      <h1>Log In</h1>
      <div className="Column">
        <label>Email</label>
        <input className="TextInput" />
      </div>
      <div className="Column">
        <label>Password</label>
        <input className="TextInput" type='password' />
      </div>
      <button className="Button">Log In</button>
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
