import './CreateAccount.css'

const CreateAccount = () => {
  return (
    <div>
      <h1>Create Account</h1>
      <div className='Row'>
        <div className="Column">
          <label>First Name</label>
          <input />
        </div>
        <div className="Column">
          <label>Last Name</label>
          <input />
        </div>
      </div>
      <div className='Row'>
        <div className="Column">
          <label>Username</label>
          <input />
        </div>
        <div className="Column">
          <label>Password</label>
          <input />
        </div>
      </div>
      <div className='Row'>
        <div className="Column">
          <label>Email</label>
          <input />
        </div>
        <div className="Column">
          <label>Major</label>
          <input />
        </div>
      </div>
      <div className="Column">
        <label>Location</label>
        <select />
      </div>
      <button>Create Account</button>
    </div>
  )
}

export default CreateAccount
