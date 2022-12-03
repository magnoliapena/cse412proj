import './Info.css'

const Info = () => {
  return (
    <div>
      <div>picture</div>
      <h3>Firstname Lastname</h3>
      <div className='Row'>
        <h3>Email:</h3>
        <h3>email@email.com</h3>
      </div>
      <div className='Row'>
        <h3>Location:</h3>
        <h3>Tempe</h3>
      </div>
      <div className='Row'>
        <h3>Major:</h3>
        <h3>Computer Science</h3>
      </div>
      <button>Edit Profile</button>
    </div>
  )
}

export default Info
