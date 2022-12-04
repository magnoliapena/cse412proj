import './CreateAccount.css'

const CreateAccount = () => {
  return (
    <div className='CreateAccount'>
      <h1>Create Account</h1>
      <div className='Row'>
        <div className="Column">
          <label>Username</label>
          <input className='TextInput' />
        </div>
        <div className="Column">
          <label>Password</label>
          <input className='TextInput' type='password' />
        </div>
      </div>
      <div className='Row'>
        <div className="Column">
          <label>Email</label>
          <input className='TextInput' />
        </div>
        <div className="Column">
          <label>Major</label>
          <input className='TextInput' />
        </div>
      </div>
      <div className="Column">
        <label>Location</label>
        <select className='TextInput'>
          <option>Select Location</option>
          <option>Tempe</option>
          <option>West</option>
          <option>Polytechnic</option>
          <option>Downtown Phoenix</option>
          <option>Online</option>
        </select>
      </div>
      <button className='Button'>Create Account</button>
    </div>
  )
}

export default CreateAccount
