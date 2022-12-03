import './Login.css'

const Login = () => {
  return (
    <div>
      <h1>Log In</h1>
      <form>
        <div className="Column">
          <label>Email</label>
          <input />
        </div>
        <div className="Column">
          <label>Password</label>
          <input />
        </div>
        <button>Log In</button>
        <p>or</p>
        <button>Create Account</button>
      </form>
    </div>
  )
}

export default Login
