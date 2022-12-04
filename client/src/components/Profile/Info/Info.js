import './Info.css'

const Info = () => {
  return (
    <div>
      <div>picture</div>
      <p>Firstname Lastname</p>
      <div className='Row'>
        <p>Email:</p>
        <p>email@email.com</p>
      </div>
      <div className='Row'>
        <p>Location:</p>
        <p>Tempe</p>
      </div>
      <div className='Row'>
        <p>Major:</p>
        <p>Computer Science</p>
      </div>
      <button className='Button'>Edit Profile</button>
    </div>
  )
}

export default Info
